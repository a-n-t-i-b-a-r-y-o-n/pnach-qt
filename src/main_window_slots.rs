use std::rc::Rc;
use qt_core::{slot, SlotNoArgs};
use qt_widgets::QApplication;

impl crate::main_window::MainWindow {
	#[slot(SlotNoArgs)]
	pub unsafe fn new_file(self: &Rc<Self>) {
		todo!()
	}

}