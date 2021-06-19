use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast, NullPtr, Ref};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs};
use qt_widgets::{QGridLayout, QGroupBox, QMessageBox, QPushButton, QPlainTextEdit};
use std::collections::HashMap;
use crate::embedded_images::{ICON, get_icon};
use crate::pane::left_pane::LeftPane;

pub struct RightPane {
	pub base:			QBox<QGroupBox>,
	pub layout:			QBox<QGridLayout>,
	pub copy_btn:		QBox<QPushButton>,
	pub save_btn:		QBox<QPushButton>,
	pub field:			QBox<QPlainTextEdit>,
}

impl StaticUpcast<QObject> for RightPane {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl RightPane {
	pub unsafe fn new() -> Rc<Self> {
		// Base group box and layout
		let group_box = QGroupBox::from_q_string(&qs("Output PNach"));
		let layout = QGridLayout::new_1a(&group_box);
		group_box.set_layout(&layout);
		// Widgets
		let copy_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::COPY), &qs(""));
		let save_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::SAVE), &qs(""));
		let field = QPlainTextEdit::new();
		// Pane object
		let mut this = Rc::new(Self {
			base: group_box,
			layout,
			copy_btn,
			save_btn,
			field
		});
		// Finish initialization
		this.initialize_pane();

		this
	}

	/// Add widgets, connect slots
	unsafe fn initialize_pane(self: &Rc<Self>) {
		// Place buttons into layout along the top.
		// Note that they're in HBox spots 1 and 2 since 0 will have the MainWindow Refresh button.
		&self.layout.add_widget_5a(&self.copy_btn, 0, 1, 1, 1);
		&self.layout.add_widget_5a(&self.save_btn, 0, 2, 1, 1);
		// Add the text input field, with auto-stretching width
		&self.layout.add_widget_5a(&self.field, 1, 0, 2, -1);
		// Connect the clicked() slots for buttons
		&self.copy_btn.clicked().connect(&self.slot_on_copy_btn_clicked());
		&self.save_btn.clicked().connect(&self.slot_on_save_btn_clicked());
	}

	/// Refresh the field
	pub fn refresh_field(self: &Rc<Self>) {

	}

}