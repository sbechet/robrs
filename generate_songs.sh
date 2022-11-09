#!/bin/bash
cargo run --bin database -- list | while read l ; do m=`echo $l | sed 's/ /_/g'` ; cargo run --bin database "$l" > "src/song_${m}.rs" ; done
