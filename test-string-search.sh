#!/bin/bash

set -e

cargo b --bin string-search-heaptrack --release
cargo t --lib