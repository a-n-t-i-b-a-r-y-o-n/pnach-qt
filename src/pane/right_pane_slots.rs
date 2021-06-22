use crate::pane::right_pane;
use std::rc::Rc;
use qt_core::{
	qs, slot,
	SlotNoArgs
};
use qt_widgets::{QFileDialog, q_file_dialog::AcceptMode};

/// Right pane slots
impl right_pane::RightPane {
	/// Use Qt to copy to field contents to the clipboard
	#[slot(SlotNoArgs)]
	pub unsafe fn on_copy_btn_clicked(self: &Rc<Self>) {
		// Select all text
		&self.field.select_all();
		// Copy selection to clipboard
		&self.field.copy();
	}
	#[slot(SlotNoArgs)]
	pub unsafe fn on_save_btn_clicked(self: &Rc<Self>) {
		// Create the file picker dialog
		let save_dialog = QFileDialog::from_q_widget_q_string(&self.base, &qs("Save PNach File..."));
		// Set the dialog to save mode
		save_dialog.set_accept_mode(AcceptMode::AcceptSave);
		// Show dialog
		save_dialog.show();
	}
}