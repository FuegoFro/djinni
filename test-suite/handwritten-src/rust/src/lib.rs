extern crate libc;
#[macro_use(mopafy)]
extern crate mopa;
extern crate encoding;

#[macro_use]
pub mod support_lib;
mod generated_rust;
pub mod generated_rust_jni;

// Hand written source
mod cpp_exception;
mod cpp_exception_impl;
mod dummy_interface_impl;
mod static_nullity_interface;
mod test_helpers;