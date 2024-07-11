//@ revisions: one two three four
//@ only-wasm32-wasip1
//@ compile-flags: --crate-type=lib
//
//
//@ [one] check-fail
//@ [one] compile-flags: -C target-feature=+relaxed-simd
//@ [two] check-fail
//@ [two] compile-flags: -C target-feature=-relaxed-simd,+relaxed-simd
//@ [three] check-fail
//@ [three] compile-flags: -C target-feature=+simd128,+relaxed-simd,-simd128
//@ [four] build-pass
//@ [four] compile-flags: -C target-feature=-simd128,+relaxed-simd -C target-feature=+simd128
