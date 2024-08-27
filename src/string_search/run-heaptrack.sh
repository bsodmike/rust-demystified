#!/bin/bash

set -e

cargo build --bin string-search-heaptrack --release
heaptrack ../../target/release/string-search-heaptrack