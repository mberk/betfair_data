use std::path::Path;

use pyo3::prelude::*;
// use std::path::PathBuf;

use super::market::{PyMarket, PyMarketsDeser};
use crate::config::Config;
use crate::file_iter::{FileIter, IntoMarketIter, MarketID};
use crate::market_source::{Adapter, MarketSource, SourceConfig, SourceItem};

#[pyclass]
pub struct ImmutAdapter {
    inner: Adapter<File>,
}

impl ImmutAdapter {
    pub fn new(source: Box<dyn MarketSource + Send>) -> Self {
        Self {
            inner: Adapter::new(source),
        }
    }
}

#[pymethods]
impl ImmutAdapter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self, py: Python) -> Option<PyObject> {
        self.inner.next().map(|f| f.into_py(py))
    }
}

#[pyclass(name = "File")]
pub struct File {
    inner: FileIter<PyMarket, ImmutableRep>,
}

#[pymethods]
impl File {
    // #[new]
    // #[args(cumulative_runner_tv = "true")]
    // fn __new__(file: PathBuf, bytes: &[u8], cumulative_runner_tv: bool) -> PyResult<Self> {
    //     Ok(Self {
    //         inner: FileIter::new(file, bytes, cumulative_runner_tv)?,
    //     })
    // }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self, py: Python) -> Option<PyObject> {
        self.inner.next(py)
    }

    fn file_name(&self) -> &Path {
        self.inner.file_name()
    }
}

impl From<(SourceItem, SourceConfig)> for File {
    fn from(s: (SourceItem, SourceConfig)) -> Self {
        Self {
            inner: FileIter::from(s),
        }
    }
}

struct ImmutableRep();
impl IntoMarketIter for ImmutableRep {
    type Market = PyMarket;
    type Deser<'a, 'de, 'py> = PyMarketsDeser<'a, 'py>;

    fn new<'a, 'de, 'py>(
        books: &'a [Py<Self::Market>],
        py: Python<'py>,
        config: Config,
    ) -> Self::Deser<'a, 'de, 'py> {
        PyMarketsDeser {
            markets: books,
            py,
            config,
        }
    }
}
impl MarketID for PyMarket {
    fn id(&self) -> &str {
        self.market_id.as_str()
    }
}
