use std::sync::Arc;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::HashSet;
use generated_rust::assorted_primitives::AssortedPrimitives;
use generated_rust::user_token::UserToken;
use generated_rust::set_record::SetRecord;
use generated_rust::primitive_list::PrimitiveList;
use generated_rust::nested_collection::NestedCollection;
use generated_rust::map_list_record::MapListRecord;
use generated_rust::client_interface::ClientInterface;
use generated_rust::color::Color;
use generated_rust::nullity_interface::NullityInterface;
use generated_rust::dummy_interface::DummyInterface;
use dummy_interface_impl::DummyInterfaceImpl;

pub fn get_set_record() -> SetRecord {
    let mut set: HashSet<String> = HashSet::new();
    set.insert("StringA".to_string());
    set.insert("StringB".to_string());
    set.insert("StringC".to_string());
    SetRecord {
        set: set,
        iset: HashSet::new(),
    }
}

pub fn check_set_record(rec: SetRecord) -> bool {
    let mut set: HashSet<String> = HashSet::new();
    set.insert("StringA".to_string());
    set.insert("StringB".to_string());
    set.insert("StringC".to_string());
    rec.set == set
}

pub fn get_primitive_list() -> PrimitiveList {
    PrimitiveList {
        list: vec!(1, 2, 3)
    }
}

pub fn check_primitive_list(pl: PrimitiveList) -> bool {
    false
    // let rust_list = PrimitiveList {
    //     list: vec!(1, 2, 3)
    // };
    // pl.list == rust_list
}

pub fn get_nested_collection() -> NestedCollection {
    NestedCollection {
        set_list: vec!(),
    }

}

pub fn check_nested_collection(nc: NestedCollection) -> bool {
    false
}

pub fn get_map() -> HashMap<String, i64> {
    HashMap::new()
}

pub fn check_map(m: HashMap<String, i64>) -> bool {
    false
}

pub fn get_empty_map() -> HashMap<String, i64> {
    HashMap::new()
}

pub fn check_empty_map(m: HashMap<String, i64>) -> bool {
    false
}

pub fn get_map_list_record() -> MapListRecord {
    MapListRecord {
        map_list: vec!(),
    }
}

pub fn check_map_list_record(m: MapListRecord) -> bool {
    false
}

pub fn check_client_interface_ascii(i: Arc<Box<ClientInterface>>) {

}

pub fn check_client_interface_nonascii(i: Arc<Box<ClientInterface>>) {

}

pub fn check_enum_map(m: HashMap<Color, String>) {

}

pub fn check_enum(c: Color) {

}

pub fn token_id(t: Option<Arc<Box<UserToken>>>) -> Option<Arc<Box<UserToken>>> {
    None
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

pub fn check_cpp_token(t: Arc<Box<UserToken>>) {

}

pub fn cpp_token_id(t: Arc<Box<UserToken>>) -> i64 {
    0
}

pub fn check_token_type(t: Arc<Box<UserToken>>, expected_type: String) -> bool {
    t.whoami() == expected_type
}

pub fn return_none() -> Option<i32> {
    None
}

pub fn assorted_primitives_id(i: AssortedPrimitives) -> AssortedPrimitives {
    return i;
}

struct NullityInterfaceImpl;

impl NullityInterface for NullityInterfaceImpl {
    fn non_null_parameters(&self, p1: Arc<Box<DummyInterface>>, p2: Arc<Box<DummyInterface>>) {

    }

    fn non_null_return(&self, should_return_null: bool) -> Arc<Box<DummyInterface>> {
        Arc::new(Box::new(DummyInterfaceImpl))
    }

    fn nullable_parameters(&self, p1: Option<Arc<Box<DummyInterface>>>, p2: Option<Arc<Box<DummyInterface>>>) {

    }

    fn nullable_return(&self, should_return_null: bool) -> Option<Arc<Box<DummyInterface>>> {
        None
    }
}

pub fn get_nullity_interface() -> Arc<Box<NullityInterface>> {
    Arc::new(Box::new(NullityInterfaceImpl))
}

pub fn check_interface_nullity_parameters(i: Arc<Box<NullityInterface>>) {

}

pub fn check_interface_nullity_return(i: Arc<Box<NullityInterface>>) {

}
