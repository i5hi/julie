#!/bin/bash -e

mkdir -p ~/.julie
cp $PWD/art/hound.ascii ~/.julie/banner.ascii

cargo build --release

cd target/release/
strip jd
strip jc

sudo cp jd /usr/bin
sudo cp jc /usr/bin


du -h jd
du -h jc
