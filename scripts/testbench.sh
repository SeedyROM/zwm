#!/bin/bash

trap 'killall zwm && kill -9 $(jobs -p %1)' 2
cargo build
Xephyr -noreset -screen 1000x600 :1 &
sleep 0.1
RUST_LOG=trace ./target/debug/zwm &
sleep 0.1
DISPLAY=:1 xclock &
DISPLAY=:1 st &
wait
