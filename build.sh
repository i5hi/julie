#!/bin/bash

cargo build --release
cd target/release/
strip jd
strip jc

sudo cp jd /usr/bin
sudo cp jc /usr/bin

du -h jd
du -h jc
