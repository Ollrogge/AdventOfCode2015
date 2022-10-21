#!/bin/bash

for i in {0..32768}
do
    dig +short flare-on$i.com
done
