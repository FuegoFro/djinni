use std::sync::Arc;
use std::boxed::Box;
use generated_rust::assorted_primitives::AssortedPrimitives;
use generated_rust::user_token::UserToken;

pub fn assorted_primitives_id(i: AssortedPrimitives) -> AssortedPrimitives {
    return i;
}

pub fn check_token_type(t: Arc<Box<UserToken>>, expected_type: String) -> bool {
    t.whoami() == expected_type
}

struct CppUserToken {
    token_type: String,
}

impl UserToken for CppUserToken {
    fn whoami(&self) -> String {
        self.token_type.clone()
    }
}

pub fn create_cpp_token() -> Arc<Box<UserToken>> {
    Arc::new(Box::new(CppUserToken {
        token_type: "C++".to_string(),
    }))
}
