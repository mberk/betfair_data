from typing import Sequence 

import unittest.mock
import tarfile
import bz2
import betfairlightweight


trading = betfairlightweight.APIClient("username", "password", "appkey")
listener = betfairlightweight.StreamListener(
    max_latency=None, lightweight=True, update_clk=False, output_queue=None, cumulative_runner_tv=True, calculate_market_tv=True
)

paths = [ 
    "data/2021_10_OctRacingAUPro.tar",
    "data/2021_11_NovRacingAUPro.tar",
    "data/2021_12_DecRacingAUPro.tar"
]

def load_tar(file_paths: Sequence[str]):
    for file_path in file_paths:
        with tarfile.TarFile(file_path) as archive:
            for file in archive:
                yield bz2.open(archive.extractfile(file)).read()
    return None

market_count = 0
update_count = 0

for i, file_obj in enumerate(load_tar(paths)):
    with unittest.mock.patch("builtins.open", lambda f, _: f):  
        stream = trading.streaming.create_historical_generator_stream(
            file_path=file_obj,
            listener=listener,
        )
        gen = stream.get_generator()

        market_count += 1
        for market_books in gen():
            for market_book in market_books:
                update_count += 1

    print(f"Market {market_count} Update {update_count}", end='\r')
print(f"Market {market_count} Update {update_count}")
