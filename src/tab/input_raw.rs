use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject};
use qt_widgets::{QGridLayout, QPlainTextEdit, QWidget};
use crate::tab::{InputTab, TabType};

pub struct TabInputRaw {
	pub base:	QBox<QWidget>,
	pub layout:	QBox<QGridLayout>,
	pub field:	QBox<QPlainTextEdit>,
}

impl StaticUpcast<QObject> for TabInputRaw {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl TabInputRaw {
	pub unsafe fn new() -> Rc<Self> {
		let base = QWidget::new_0a();
		let layout = QGridLayout::new_1a(&base);
		base.set_layout(&layout);

		let field = QPlainTextEdit::new();

		let this = Rc::new(Self {
			base,
			layout,
			field,
		});
		this.initialize_tab();

		this
	}

	unsafe fn initialize_tab(self: &Rc<Self>) {
		&self.layout.add_widget_5a(&self.field, 0, 0, -1, -1);
	}

}

impl InputTab for TabInputRaw {
	fn get_tab_type() -> TabType {
		TabType::RAW
	}

	unsafe fn get_raw_codes(self: &Rc<Self>) -> String {
		String::from(self.field.to_plain_text().as_raw_ref())
	}
}