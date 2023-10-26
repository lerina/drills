#!/bin/sh

## pre-req a web server
# cargo install http

## exit on error and  prints each executed command
set -ex

## remove old pkg somethings dont update after modifications
rm -fr www/pkg

## compile for plain vanilla no javascript framework 
wasm-pack build --target web --out-dir www/pkg  
#--color=always 2>&1 | less -R

## display link for easy access
echo "Serving at: http://127.0.0.1:8080/html/"

## run the web server
http -a 127.0.0.1 -p 8080 www
