#!/bin/bash
mkdir -p src/song
cargo run --bin siddump -- list | while read l ; do m=`echo $l | sed 's/ /_/g'` ; cargo run --bin siddump "$l" > "src/song/song_${m}.rs" ; done
basename -s '.rs' -a src/song/song*rs | while read l ; do echo "#[allow(non_snake_case)]" ; echo pub mod ${l}\; ; done > src/song/mod.rs