#!/bin/bash -e


ls */Cargo.toml | awk -F/ '{print $1}' | while read n; do
	echo $n
	cd $n
	cargo build
	cargo build --no-default-features --features tstd
	cd ../
done
