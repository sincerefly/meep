#!/bin/bash

sudo docker save -o meep.tar meep:latest
sudo tar zcvf meep.tar.bz2 meep.tar
