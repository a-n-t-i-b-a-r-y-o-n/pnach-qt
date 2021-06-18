use crate::main_window;
use std::rc::Rc;
use qt_core::{
	qs, slot, ContextMenuPolicy,
	QBox, QObject, QPoint,
	SlotNoArgs
};
use qt_widgets::{QApplication, QMenu, QMessageBox, SlotOfQPoint};
use cpp_core::{NullPtr, Ref};
use crate::pane::right_pane;

/// Right pane slots
impl right_pane::RightPane {
	#[slot(SlotNoArgs)]
	pub unsafe fn on_refresh_btn_clicked(self: &Rc<Self>) {
		&self.field.set_plain_text(&qs("Refreshing..."));
	}
	#[slot(SlotNoArgs)]
	pub unsafe fn on_copy_btn_clicked(self: &Rc<Self>) {
		&self.field.set_plain_text(&qs("Copying..."));
	}
	#[slot(SlotNoArgs)]
	pub unsafe fn on_save_btn_clicked(self: &Rc<Self>) {
		&self.field.set_plain_text(&qs("Saving..."));
	}
}