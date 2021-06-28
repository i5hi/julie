#!/bin/bash

cargo build --release
cd target/release/
strip jd
strip jc
cp jd /usr/bin
cp jc /usr/bin

du -h jd
du -h jc
