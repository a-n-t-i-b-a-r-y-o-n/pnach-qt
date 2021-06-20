use crate::pane::right_pane;
use std::rc::Rc;
use qt_core::{
	qs, slot,
	SlotNoArgs
};

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
		&self.field.set_plain_text(&qs("Saving..."));
	}
}