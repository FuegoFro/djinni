use std::sync::Arc;
use std::boxed::Box;
use generated_rust::dummy_interface::DummyInterface;
use dummy_interface_impl::DummyInterfaceImpl;

pub fn non_null_parameters(p1: Arc<Box<DummyInterface>>, p2: Arc<Box<DummyInterface>>) {

}

pub fn non_null_return(should_return_null: bool) -> Arc<Box<DummyInterface>> {
    assert!(!should_return_null, "Cannot return null");
    Arc::new(Box::new(DummyInterfaceImpl))
}

pub fn nullable_parameters(p1: Option<Arc<Box<DummyInterface>>>, p2: Option<Arc<Box<DummyInterface>>>) {

}

pub fn nullable_return(should_return_null: bool) -> Option<Arc<Box<DummyInterface>>> {
    if should_return_null {
        None
    } else {
        Some(Arc::new(Box::new(DummyInterfaceImpl)))
    }
}
