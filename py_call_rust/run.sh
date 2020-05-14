#!/usr/bin/bash

rustc rust_math.rs
python test.py

rustc -A warnings main.rs

./main
