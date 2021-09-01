#!/bin/bash

svd2rust -i W7500x.svd

rm -rf src

form -i lib.rs -o src/ && rm lib.rs

cargo fmt