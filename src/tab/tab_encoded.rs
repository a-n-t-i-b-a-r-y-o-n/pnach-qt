use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject};
use qt_widgets::{QFormLayout, QGridLayout, QPlainTextEdit, QPushButton, QWidget};
use crate::tab::{InputTab, TabType};

pub struct TabInputEncoded {
	pub base:			QBox<QWidget>,
	pub layout:			QBox<QGridLayout>,
	pub field:			QBox<QPlainTextEdit>,
	pub panel:			QBox<QWidget>,
	pub panel_layout:	QBox<QFormLayout>,
}

impl StaticUpcast<QObject> for TabInputEncoded {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl TabInputEncoded {
	pub unsafe fn new() -> Rc<Self> {
		let base = QWidget::new_0a();
		let layout = QGridLayout::new_1a(&base);
		base.set_layout(&layout);

		let field = QPlainTextEdit::new();
		let panel = QWidget::new_0a();
		let panel_layout = QFormLayout::new_1a(&panel);
		panel.set_layout(&panel_layout);

		let this = Rc::new(Self {
			base,
			layout,
			field,
			panel,
			panel_layout,
		});
		this.initialize_panel();
		this.initialize_tab();

		this
	}

	unsafe fn initialize_tab(self: &Rc<Self>) {
		&self.layout.add_widget_5a(&self.field, 0, 0, -1, 1);
		&self.layout.add_widget_5a(&self.panel, 0, 1, -1, 1);
	}

	unsafe fn initialize_panel(self: &Rc<Self>) {
		let button = QPushButton::from_q_string(&qs("Button!"));
		&self.panel_layout.add_widget(&button);
	}

}

impl InputTab for TabInputEncoded {
	fn get_tab_type() -> TabType {
		TabType::ENCODED
	}

	unsafe fn get_raw_codes(self: &Rc<Self>) -> String {
		String::from(self.field.to_plain_text().as_raw_ref())
	}
}