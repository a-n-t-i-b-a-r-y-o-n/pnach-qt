use std::rc::Rc;
use qt_core::{qs, QVariant};
use cpp_core::CppBox;
use std::any::Any;

pub mod tab_raw;
pub mod tab_encoded;

enum TabType { RAW, ENCODED }

pub unsafe fn identify_tab_type(tab: &CppBox<QVariant>) {
	if !tab.is_null() && tab.is_valid() {

	}
	println!("{:?}", &tab.as_raw_ref().type_());
}