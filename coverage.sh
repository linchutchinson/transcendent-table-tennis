#!/usr/bin/env sh

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

mkdir -p target/coverage/html
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov

touch this_is_a_dumb_hack_but_i_dont_know_bash.profraw
rm *.profraw
rm **/*.profraw

touch electric_boogaloo.profdata
rm *.profdata
