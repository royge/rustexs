#!/usr/bin/env bash

cd tempconvert && cargo test --verbose --workspace && cd ../
cd fibo && cargo test --verbose --workspace && cd ../
cd xmasdays && cargo test --verbose --workspace && cd ../
cd measures && cargo test --verbose --workspace && cd ../
cd piglatin && cargo test --verbose --workspace && cd ../
cd coolhr && cargo test --verbose --workspace && cd ../
cd minigrep && cargo test --verbose --workspace && cd ../
cd cacher && cargo test --verbose --workspace && cd ../
cd klosur && cargo test --verbose --workspace && cd ../
cd iterators && cargo test --verbose --workspace && cd ../
cd smartptr && cargo test --verbose --workspace && cd ../
cd smartptr && cargo run && cd ../
cd blog && cargo run && cd ../
cd gui && cargo run && cd ../
cd patmat && cargo run && cd ../
cd fnptr && cargo run && cd ../
cd neotype && cargo run && cd ../
cd overloaded && cargo run && cd ../
cd makru && cargo run && cd ../
cd hello && cargo test && cd ../
