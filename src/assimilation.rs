use crate::main_window::MainWindow;
use std::rc::Rc;

impl MainWindow {
	unsafe fn get_left_pane_raw_codes(self: &Rc<Self>) {
		for tab in self.left_pane.tab_widget.children().iter() {
			println!("Child: {:?}", tab);
		}
	}
}