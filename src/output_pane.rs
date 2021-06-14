use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject};
use qt_widgets::{QGridLayout, QGroupBox, QPushButton, QPlainTextEdit};
use std::collections::HashMap;
use crate::embedded_images::{ICON, get_icon};

pub struct OutputPane {
	pub base:			QBox<QGroupBox>,
	pub layout:			QBox<QGridLayout>,
	pub field:			QBox<QPlainTextEdit>,
	pub buttons:		HashMap<String, QBox<QPushButton>>,
}

impl StaticUpcast<QObject> for OutputPane {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl OutputPane {
	pub unsafe fn new() -> Rc<Self> {
		// Base group box and layout
		let group_box = QGroupBox::from_q_string(&qs("Output PNach"));
		let layout = QGridLayout::new_1a(&group_box);
		group_box.set_layout(&layout);
		// Widgets
		let field = QPlainTextEdit::new();
		let buttons = HashMap::new();
		// Pane object
		let this = Rc::new(Self {
			base: group_box,
			layout,
			field,
			buttons
		});
		// Finish initialization
		this.initialize_pane();

		this
	}

	/// Add widgets, connect slots
	unsafe fn initialize_pane(self: &Rc<Self>) {
		&self.layout.add_widget_5a(&self.field, 0, 0, 2, -1);

		let refresh_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::REFRESH), &qs(""));
		let copy_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::COPY), &qs(""));
		let save_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::SAVE), &qs(""));

		&self.layout.add_widget_5a(&refresh_btn, 2, 0, 1, 1);
		&self.layout.add_widget_5a(&copy_btn, 2, 1, 1, 1);
		&self.layout.add_widget_5a(&save_btn, 2, 2, 1, 1);
	}
}