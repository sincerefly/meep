#!/bin/bash

sudo docker run -v $PWD:/volume -t clux/muslrust cargo build --release
#sudo docker run -v $PWD:/volume -t muslrust-cn cargo build --release
