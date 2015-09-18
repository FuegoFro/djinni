use std::sync::Arc;
use std::boxed::Box;
use generated_rust::cpp_exception::CppException;
use cpp_exception_impl::CppExceptionImpl;

pub fn get() -> Arc<Box<CppException>> {
    Arc::new(Box::new(CppExceptionImpl))
}