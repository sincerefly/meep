#!/bin/bash

cp ../target/x86_64-unknown-linux-musl/release/meep .

sudo docker build -t meep:latest .

