#!/bin/bash

rm -rf dist/

rm -rf static/

rm -rf target/dx/

dx build --release --ssg

mkdir dist/

cp -r target/dx/dioxus-prototype/release/web/public/* dist/

cp -r static/* dist/

cp -r merge/* dist/

cd dist/

http-server -c-1 -o
