cargo clean -p experiment2;
cargo rustc -p experiment2 -- -Zprint-type-sizes > log.txt
