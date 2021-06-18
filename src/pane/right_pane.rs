use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast, NullPtr};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs};
use qt_widgets::{QGridLayout, QGroupBox, QMessageBox, QPushButton, QPlainTextEdit};
use std::collections::HashMap;
use crate::embedded_images::{ICON, get_icon};

pub struct RightPane {
	pub base:			QBox<QGroupBox>,
	pub layout:			QBox<QGridLayout>,
	pub field:			QBox<QPlainTextEdit>,
	pub refresh_btn:	QBox<QPushButton>,
	pub copy_btn:		QBox<QPushButton>,
	pub save_btn:		QBox<QPushButton>,
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
		let field = QPlainTextEdit::new();
		let refresh_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::REFRESH), &qs(""));
		let copy_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::COPY), &qs(""));
		let save_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::SAVE), &qs(""));
		// Pane object
		let mut this = Rc::new(Self {
			base: group_box,
			layout,
			field,
			refresh_btn,
			copy_btn,
			save_btn
		});
		// Finish initialization
		this.initialize_pane();

		this
	}

	/// Add widgets, connect slots
	unsafe fn initialize_pane(self: &Rc<Self>) {
		&self.layout.add_widget_5a(&self.field, 0, 0, 2, -1);

		&self.refresh_btn.clicked().connect(&self.slot_on_refresh_btn_clicked());
		&self.copy_btn.clicked().connect(&self.slot_on_copy_btn_clicked());
		&self.save_btn.clicked().connect(&self.slot_on_save_btn_clicked());

		&self.layout.add_widget_5a(&self.refresh_btn, 2, 0, 1, 1);
		&self.layout.add_widget_5a(&self.copy_btn, 2, 1, 1, 1);
		&self.layout.add_widget_5a(&self.save_btn, 2, 2, 1, 1);
	}


}