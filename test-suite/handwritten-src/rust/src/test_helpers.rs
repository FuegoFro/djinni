use std::rc::Rc;
use std::cell::RefCell;
use generated_rust::assorted_primitives::AssortedPrimitives;
use generated_rust::user_token::UserToken;

pub fn assorted_primitives_id(i: AssortedPrimitives) -> AssortedPrimitives {
    return i;
}

// pub fn check_token_type(t: Rc<RefCell<UserToken>>, expected_type: String) {
//     t.borrow().borrow().whoami() != expected_type
// }
