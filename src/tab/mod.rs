use std::rc::Rc;

pub mod input_raw;
pub mod input_encoded;

pub enum TabType { RAW, ENCODED }

pub trait InputTab {
	fn get_tab_type() -> TabType;
	unsafe fn get_raw_codes(self: &Rc<Self>) -> String;
}