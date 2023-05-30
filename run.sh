#!/bin/sh
cargo build --release
sudo cp ./judson.service /etc/systemd/system/judson.service
sudo mv $PWD/target/release/judson /bin
# judson
