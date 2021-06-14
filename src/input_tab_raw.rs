use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{QBox, QObject};
use qt_widgets::{QGridLayout, QPlainTextEdit, QWidget};

pub struct InputTab_Raw {
	pub base:	QBox<QWidget>,
	pub layout:	QBox<QGridLayout>,
	pub field:	QBox<QPlainTextEdit>,
}

impl StaticUpcast<QObject> for InputTab_Raw {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl InputTab_Raw {
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