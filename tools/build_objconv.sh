#!/bin/bash

git clone git@github.com:vertis/objconv.git
pushd objconv
	g++ -o objconv -O2 src/*.cpp
	sudo cp objconv /usr/local/bin
popd
