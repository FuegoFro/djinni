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
