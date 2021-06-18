use crate::main_window;
use std::rc::Rc;
use qt_core::{qs, slot, SlotNoArgs};
use qt_widgets::{QApplication, QMessageBox};
use cpp_core::NullPtr;

impl main_window::MainWindow {
	#[slot(SlotNoArgs)]
	pub unsafe fn new_file(self: &Rc<Self>) {
		QMessageBox::information_q_widget2_q_string(&self.central, &qs("Example"), &qs("MessageBox"));
	}
}