#!/bin/bash -e

mkdir -p ~/.julie/.keys
cp $PWD/art/hound.ascii ~/.julie/banner.ascii
cp $PWD/config.json ~/.julie/config.json
cp $PWD/email.html ~/.julie/email.html

cargo build --release

cd target/release/
strip jd
strip jc

sudo cp jd /usr/bin
sudo cp jc /usr/bin


du -h jd
du -h jc
