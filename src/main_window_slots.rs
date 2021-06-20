use crate::main_window;
use crate::tab::*;
use std::rc::Rc;
use qt_core::{qs, slot, SlotNoArgs};
use qt_widgets::{QMessageBox};

impl main_window::MainWindow {
	/// Start over with a clean slate in a new file
	#[slot(SlotNoArgs)]
	pub unsafe fn new_file(self: &Rc<Self>) {
		QMessageBox::information_q_widget2_q_string(&self.central, &qs("Example"), &qs("MessageBox"));
	}

	/// Right pane refresh button clicked()
	#[slot(SlotNoArgs)]
	pub unsafe fn refresh_clicked(self: &Rc<Self>) {
		for i in 0..self.left_pane.tab_widget.tab_bar().count() {
			//&self.right_pane.field.append_plain_text(&self.left_pane.tab_widget.tab_bar().tab_text(i));
			identify_tab_type(&self.left_pane.tab_widget.tab_bar().tab_data(i));
		}
		//&self.right_pane.field.set_plain_text(&qs(format!("{:?}", &self.left_pane.tab_widget.children())));
	}
}