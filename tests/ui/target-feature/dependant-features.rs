//@ build-fail
//@ only-wasm32-wasip1
//@ compile-flags: --crate-type=lib

pub fn test() {
    #[target_feature(enable = "relaxed-simd")]
    //~^ ERROR the target feature relaxed-simd requires simd128 to be enabled
    unsafe fn inner() {}

    unsafe {
        foo();
        bar();
        baz();
        inner();
    }
}

#[target_feature(enable = "relaxed-simd")]
//~^ ERROR the target feature relaxed-simd requires simd128 to be enabled
unsafe fn foo() {}

#[target_feature(enable = "relaxed-simd,simd128")]
unsafe fn bar() {}

#[target_feature(enable = "relaxed-simd")]
#[target_feature(enable = "simd128")]
unsafe fn baz() {}
