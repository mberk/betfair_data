use pyo3::prelude::*;
use serde::{
    de::{self, DeserializeSeed, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::borrow::Cow;
use std::fmt;

use crate::datetime::DateTimeString;
use crate::enums::{MarketBettingType, MarketStatus};
use crate::ids::{EventID, EventTypeID};
use crate::mutable::runner::{PyRunner, PyRunnerDefSeq};
use crate::strings::{FixedSizeString, StringSetExtNeq};
use crate::config::Config;

#[derive(Default, Clone)]
pub struct MarketDefinition {
    pub bet_delay: u16,
    pub bsp_market: bool,
    pub bsp_reconciled: bool,
    pub complete: bool,
    pub cross_matching: bool,
    pub discount_allowed: bool,
    pub each_way_divisor: Option<f64>,
    pub event_id: EventID,
    pub event_name: Option<String>,
    pub event_type_id: EventTypeID,
    pub in_play: bool,
    pub market_base_rate: f32,
    pub market_type: String,
    pub market_name: Option<String>,
    pub number_of_active_runners: u16,
    pub number_of_winners: u8,
    pub persistence_enabled: bool,
    pub runners_voidable: bool,
    pub timezone: String,
    pub turn_in_play_enabled: bool,
    pub venue: Option<String>,
    pub version: u64,
    pub status: MarketStatus,
    pub betting_type: MarketBettingType,
    pub market_time: DateTimeString,
    pub open_date: DateTimeString,
    pub suspend_time: Option<DateTimeString>,
    pub settled_time: Option<DateTimeString>,
    pub country_code: FixedSizeString<2>,
    pub regulators: Vec<String>,
}

// Used for serializing in place over the mc marketDefinition object
pub struct PyMarketDefinition<'a, 'py> {
    pub def: &'a mut MarketDefinition,
    pub runners: &'a mut Vec<Py<PyRunner>>,
    pub config: Config,
    pub img: bool,
    pub py: Python<'py>,
}
impl<'de, 'a, 'py> DeserializeSeed<'de> for PyMarketDefinition<'a, 'py> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            BetDelay,
            BettingType,
            BspMarket,
            BspReconciled,
            Complete,
            CountryCode,
            CrossMatching,
            DiscountAllowed,
            EachWayDivisor,
            EventId,
            EventName,
            EventTypeId,
            InPlay,
            KeyLineDefiniton,
            LineMaxUnit,
            LineMinUnit,
            LineInterval,
            MarketBaseRate,
            MarketTime,
            MarketType,
            Name,
            NumberOfActiveRunners,
            NumberOfWinners,
            OpenDate,
            PersistenceEnabled,
            PriceLadderDefinition,
            RaceType,
            Regulators,
            Runners,
            RunnersVoidable,
            SettledTime,
            Status,
            SuspendTime,
            Timezone,
            TurnInPlayEnabled,
            Venue,
            Version,
        }

        struct PyMarketDefinitionVisitor<'a, 'py> {
            def: &'a mut MarketDefinition,
            runners: &'a mut Vec<Py<PyRunner>>,
            config: Config,
            img: bool,
            py: Python<'py>,
        }
        impl<'de, 'a, 'py> Visitor<'de> for PyMarketDefinitionVisitor<'a, 'py> {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("")
            }

            fn visit_map<V>(mut self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::BspMarket => self.def.bsp_market = map.next_value()?,
                        Field::TurnInPlayEnabled => {
                            self.def.turn_in_play_enabled = map.next_value()?
                        }
                        Field::InPlay => self.def.in_play = map.next_value()?,
                        Field::PersistenceEnabled => {
                            self.def.persistence_enabled = map.next_value()?
                        }
                        Field::BspReconciled => self.def.bsp_reconciled = map.next_value()?,
                        Field::Complete => self.def.complete = map.next_value()?,
                        Field::CrossMatching => self.def.cross_matching = map.next_value()?,
                        Field::RunnersVoidable => self.def.runners_voidable = map.next_value()?,
                        Field::DiscountAllowed => self.def.discount_allowed = map.next_value()?,
                        Field::Timezone => {
                            self.def.timezone.set_if_ne(map.next_value::<&str>()?);
                        }
                        Field::Name => {
                            self.def
                                .market_name
                                .set_if_ne(map.next_value::<Cow<str>>()?);
                        }
                        Field::EventName => {
                            self.def.event_name.set_if_ne(map.next_value::<Cow<str>>()?);
                        }
                        Field::CountryCode => {
                            self.def.country_code = map.next_value::<FixedSizeString<2>>()?;
                        }
                        Field::Venue => {
                            self.def.venue.set_if_ne(map.next_value::<Cow<str>>()?);
                        }
                        Field::Status => self.def.status = map.next_value()?,
                        Field::MarketBaseRate => {
                            self.def.market_base_rate = map.next_value::<f32>()?
                        }
                        Field::NumberOfWinners => {
                            self.def.number_of_winners = map.next_value::<f32>()? as u8
                        }
                        Field::NumberOfActiveRunners => {
                            self.def.number_of_active_runners = map.next_value()?
                        }
                        Field::BetDelay => self.def.bet_delay = map.next_value()?,
                        Field::EventId => {
                            self.def.event_id = map
                                .next_value::<&str>()?
                                .parse()
                                .map_err(de::Error::custom)?;
                        }
                        Field::EventTypeId => {
                            self.def.event_type_id = map
                                .next_value::<&str>()?
                                .parse()
                                .map_err(de::Error::custom)?;
                        }
                        Field::Version => self.def.version = map.next_value()?,
                        Field::Runners => map.next_value_seed(PyRunnerDefSeq {
                            runners: self.runners,
                            config: self.config,
                            img: self.img,
                            py: self.py,
                        })?,
                        Field::MarketType => {
                            self.def.market_type.set_if_ne(map.next_value::<&str>()?);
                        }
                        Field::BettingType => self.def.betting_type = map.next_value()?,
                        Field::EachWayDivisor => {
                            self.def.each_way_divisor = Some(map.next_value::<f64>()?)
                        }
                        Field::MarketTime => {
                            let s = map.next_value::<&str>()?;
                            if &self.def.market_time != s {
                                let dt = DateTimeString::new(s).map_err(de::Error::custom)?;

                                self.def.market_time = dt;
                            }
                        }
                        Field::SuspendTime => {
                            let s = map.next_value::<&str>()?;
                            if self.def.suspend_time.contains(&s) {
                                let dt = DateTimeString::new(s).map_err(de::Error::custom)?;
                                self.def.suspend_time = Some(dt);
                            }
                        }
                        Field::SettledTime => {
                            let s = map.next_value::<&str>()?;
                            if self.def.settled_time.contains(&s) {
                                let dt = DateTimeString::new(s).map_err(de::Error::custom)?;
                                self.def.settled_time = Some(dt);
                            }
                        }
                        Field::OpenDate => {
                            let s = map.next_value::<&str>()?;
                            if &self.def.open_date != s {
                                let dt = DateTimeString::new(s).map_err(de::Error::custom)?;
                                self.def.open_date = dt;
                            }
                        }
                        Field::Regulators => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }

                        // after searching over 200k markets, I cant find these values in any data sets :/
                        Field::RaceType => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                            // panic!("{} {}", self.0.source, self.0.file);
                        }
                        Field::KeyLineDefiniton => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                            // panic!("{} {}", self.0.source, self.0.file);
                        }
                        Field::PriceLadderDefinition => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                            // panic!("{} {}", self.0.source, self.0.file);
                        }
                        Field::LineMaxUnit => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                            // panic!("{} {}", self.0.source, self.0.file);
                        }
                        Field::LineMinUnit => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                            // panic!("{} {}", self.0.source, self.0.file);
                        }
                        Field::LineInterval => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                            // panic!("{} {}", self.0.source, self.0.file);
                        }
                    }
                }
                Ok(())
            }
        }

        const FIELDS: &[&str] = &[
            "keyLineDefiniton",
            "priceLadderDefinition",
            "raceType",
            "lineMaxUnit",
            "lineMinUnit",
            "lineInterval",
            "bspMarket",
            "turnInPlayEnabled",
            "persistenceEnabled",
            "marketBaseRate",
            "eventId",
            "eventTypeId",
            "numberOfWinners",
            "bettingType",
            "marketType",
            "marketTime",
            "suspendTime",
            "bspReconciled",
            "complete",
            "inPlay",
            "crossMatching",
            "runnersVoidable",
            "numberOfActiveRunners",
            "betDelay",
            "status",
            "runners",
            "regulators",
            "countryCode",
            "discountAllowed",
            "timezone",
            "openDate",
            "version",
            "name",
            "eventName",
            "venue",
            "settledTime",
            "eachWayDivisor",
        ];
        deserializer.deserialize_struct(
            "MarketDefinition",
            FIELDS,
            PyMarketDefinitionVisitor {
                def: self.def,
                runners: self.runners,
                config: self.config,
                img: self.img,
                py: self.py,
            },
        )
    }
}