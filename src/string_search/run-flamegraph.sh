#!/bin/bash

set -e

cargo b --bin string-search-heaptrack --release
cargo t --lib
cargo flamegraph --root --flamechart --verbose --bin string-search-heaptrack