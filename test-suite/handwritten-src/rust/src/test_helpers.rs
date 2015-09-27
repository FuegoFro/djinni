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
    let rust_list = PrimitiveList {
        list: vec!(1, 2, 3)
    };
    pl.list == rust_list.list
}

fn make_test_nested_collection() -> NestedCollection {
    let mut set1 = HashSet::new();
    set1.insert("String1".to_string());
    set1.insert("String2".to_string());
    let mut set2 = HashSet::new();
    set2.insert("StringA".to_string());
    set2.insert("StringB".to_string());
    NestedCollection {
        set_list: vec!(set1, set2),
    }
}

pub fn get_nested_collection() -> NestedCollection {
    make_test_nested_collection()
}

pub fn check_nested_collection(nc: NestedCollection) -> bool {
    nc.set_list == make_test_nested_collection().set_list
}

fn make_test_map() -> HashMap<String, i64>{
    let mut map = HashMap::new();
    map.insert("String1".to_string(), 1);
    map.insert("String2".to_string(), 2);
    map.insert("String3".to_string(), 3);
    map
}

pub fn get_map() -> HashMap<String, i64> {
    make_test_map()
}

pub fn check_map(m: HashMap<String, i64>) -> bool {
    m == make_test_map()
}

pub fn get_empty_map() -> HashMap<String, i64> {
    HashMap::new()
}

pub fn check_empty_map(m: HashMap<String, i64>) -> bool {
    m.len() == 0
}

pub fn get_map_list_record() -> MapListRecord {
    MapListRecord {
        map_list: vec!(make_test_map()),
    }
}

pub fn check_map_list_record(m: MapListRecord) -> bool {
    m.map_list.len() == 1 && m.map_list[0] == make_test_map()
}

const HELLO_WORLD: &'static str = "Hello World!";
const NON_ASCII: &'static str = "Non-ASCII / 非 ASCII 字符";

pub fn check_client_interface_ascii(i: Arc<Box<ClientInterface>>) {
    let returned_record = i.get_record(5, HELLO_WORLD.to_string(), None);
    if returned_record.content != HELLO_WORLD {
        panic!("Expected String: {} Actual: {}", HELLO_WORLD, returned_record.content);
    }
}

pub fn check_client_interface_nonascii(i: Arc<Box<ClientInterface>>) {
    let returned_record = i.get_record(5, NON_ASCII.to_string(), None);
    if returned_record.content != NON_ASCII {
        panic!("Expected String: {} Actual: {}", NON_ASCII, returned_record.content);
    }
}

pub fn check_enum_map(m: HashMap<Color, String>) {
    let mut expected = HashMap::new();
    expected.insert(Color::Red, "red".to_string());
    expected.insert(Color::Orange, "orange".to_string());
    expected.insert(Color::Yellow, "yellow".to_string());
    expected.insert(Color::Green, "green".to_string());
    expected.insert(Color::Blue, "blue".to_string());
    expected.insert(Color::Indigo, "indigo".to_string());
    expected.insert(Color::Violet, "violet".to_string());

    assert!(m == expected);
}

pub fn check_enum(_c: Color) {
    // Stub
}

pub fn token_id(t: Option<Arc<Box<UserToken>>>) -> Option<Arc<Box<UserToken>>> {
    t
}

struct RustUserToken {
    token_type: String,
}

impl UserToken for RustUserToken {
    fn whoami(&self) -> String {
        self.token_type.clone()
    }
}

pub fn create_cpp_token() -> Arc<Box<UserToken>> {
    Arc::new(Box::new(RustUserToken {
        token_type: "C++".to_string(),
    }))
}

pub fn check_cpp_token(t: Arc<Box<UserToken>>) {
    assert!(t.is::<RustUserToken>());
}

pub fn cpp_token_id(_t: Arc<Box<UserToken>>) -> i64 {
    panic!() // Unclear that this is used anywhere
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

pub fn id_binary(b: Vec<u8>) -> Vec<u8> {
    b
}

struct NullityInterfaceImpl;

impl NullityInterface for NullityInterfaceImpl {
    fn non_null_parameters(&self, _p1: Arc<Box<DummyInterface>>, _p2: Arc<Box<DummyInterface>>) {}

    fn non_null_return(&self, _should_return_null: bool) -> Arc<Box<DummyInterface>> {
        // TODO(rustgen) - fill this out once we have catch panic
        Arc::new(Box::new(DummyInterfaceImpl))
    }

    fn nullable_parameters(&self, _p1: Option<Arc<Box<DummyInterface>>>, _p2: Option<Arc<Box<DummyInterface>>>) {}

    fn nullable_return(&self, should_return_null: bool) -> Option<Arc<Box<DummyInterface>>> {
        if should_return_null {
            None
        } else {
            Some(Arc::new(Box::new(DummyInterfaceImpl)))
        }
    }
}

pub fn get_nullity_interface() -> Arc<Box<NullityInterface>> {
    Arc::new(Box::new(NullityInterfaceImpl))
}

pub fn check_interface_nullity_parameters(i: Arc<Box<NullityInterface>>) {
    // These tests aren't strictly necessary since nullity is handled by
    // the language (in that there's no such thing as null). But it doesn't
    // hurt to have them anyway.

    // Call Java object instance from Rust
    // Nullable parameters
    let dummy_interface: Arc<Box<DummyInterface>> = Arc::new(Box::new(DummyInterfaceImpl));
    // The following calls should succeed
    i.nullable_parameters(Some(dummy_interface.clone()), Some(dummy_interface.clone()));
    i.nullable_parameters(Some(dummy_interface.clone()), None);
    i.nullable_parameters(None, Some(dummy_interface.clone()));
    i.nullable_parameters(None, None);

    // Nonnull parameters
    let dummy_interface: Arc<Box<DummyInterface>> = Arc::new(Box::new(DummyInterfaceImpl));
    // The following call should succeed
    i.non_null_parameters(dummy_interface.clone(), dummy_interface.clone());
    // The following calls won't compile
    /*
    i.non_null_parameters(dummy_interface, None);
    i.non_null_parameters(None, dummy_interface);
    i.non_null_parameters(None, None);
    */

}

pub fn check_interface_nullity_return(i: Arc<Box<NullityInterface>>) {
    // Call Java object instance from Rust
    // Should succeed
    i.nullable_return(false /* should return null */);
    i.nullable_return(true /* should return null */);
    i.non_null_return(false /* should return null */);
    // Should fail
    // TODO(rustgen) - uncomment once we have a good story for propagating exceptions
    // i.non_null_return(true /* should return null */);
}
