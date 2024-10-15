//@ compile-flags: -Copt-level=s --target=avr-unknown-gnu-atmega328 -C panic=abort
//@ needs-llvm-components: avr
//@ assembly-output: emit-asm

#![feature(
    no_core,
    lang_items,
    intrinsics,
    rustc_attrs,
    arbitrary_self_types,
    asm_experimental_arch
)]
#![crate_type = "rlib"]
#![no_core]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}

use minicore::ptr;

// CHECK-LABEL: pin_toggling
// CHECK: ldi [[REG_1:r[0-9]+]], 1
// CHECK: ldi [[REG_2:r[0-9]+]], 2
// CHECK: .LBB0_1:
// CHECK-NEXT: out 5, [[REG_1]]
// CHECK-NEXT: call delay
// CHECK-NEXT: out 5, [[REG_2]]
// CHECK-NEXT: call delay
// CHECK-NEXT: rjmp .LBB0_1
#[no_mangle]
pub fn pin_toggling() {
    let port_b = 0x25 as *mut u8; // the I/O-address of PORTB
    loop {
        unsafe { ptr::write_volatile(port_b, 1) };
        delay(500_0000);
        unsafe { ptr::write_volatile(port_b, 2) };
        delay(500_0000);
    }
}

#[inline(never)]
#[no_mangle]
fn delay(_: u32) {
    unsafe { asm!("nop") };
}

// FIXME: replace with proper minicore once available (#130693)
mod minicore {
    #[lang = "sized"]
    pub trait Sized {}

    #[lang = "copy"]
    pub trait Copy {}
    impl Copy for u32 {}
    impl Copy for &u32 {}
    impl<T: ?Sized> Copy for *mut T {}

    pub mod ptr {
        #[inline]
        #[rustc_diagnostic_item = "ptr_write_volatile"]
        pub unsafe fn write_volatile<T>(dst: *mut T, src: T) {
            extern "rust-intrinsic" {
                #[rustc_nounwind]
                pub fn volatile_store<T>(dst: *mut T, val: T);
            }
            unsafe { volatile_store(dst, src) };
        }
    }
}
