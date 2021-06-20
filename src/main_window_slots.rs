use crate::main_window;
use crate::tab;
use std::rc::Rc;
use qt_core::{qs, slot, QString, SlotNoArgs};
use qt_widgets::QMessageBox;

impl main_window::MainWindow {
	/// Start over with a clean slate in a new file
	#[slot(SlotNoArgs)]
	pub unsafe fn new_file(self: &Rc<Self>) {
		QMessageBox::information_q_widget2_q_string(&self.central, &qs("Example"), &qs("MessageBox"));
	}

	/// Right pane refresh button clicked()
	#[slot(SlotNoArgs)]
	pub unsafe fn refresh_clicked(self: &Rc<Self>) {
		// Clear field contents
		&self.right_pane.field.clear();
		// Iterate left pane tabs
		for i in 0..self.left_pane.tab_widget.tab_bar().count() {
			// Get main widget holding the content of the tab at this index
			let content = &self.left_pane.tab_widget.widget(i);
			// Decode and extract codes from this tab
			let codes = tab::get_raw_codes(content);
			if !codes.is_empty() {
				// Add a comment header indicating the tab number (1-indexed for users' sake)
				&self.right_pane.field.append_plain_text(&qs(format!("// Tab {}", i+1)));
				// Append codes
				// TODO: Add this to a working PNachCode obj, then append at the end
				&self.right_pane.field.append_plain_text(&qs(codes));
			}
		}
	}
}