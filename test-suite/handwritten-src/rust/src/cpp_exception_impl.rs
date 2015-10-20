use generated_rust::cpp_exception::CppException;

pub struct CppExceptionImpl;

impl CppException for CppExceptionImpl {
    fn throw_an_exception(&self) -> i32 {
        panic!("Exception Thrown");
    }
}