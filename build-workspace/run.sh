#!/bin/bash

sudo docker run -d --name meep -p 3000:3000 -e MEEP_IP="0.0.0.0" -e MEEP_PORT=3000 -e PUB_URL="http://192.168.3.189:3000" -e SAVE_DIR="./meep-data" meep:latest /bin/sh -c 'cd /opt; ./meep'
