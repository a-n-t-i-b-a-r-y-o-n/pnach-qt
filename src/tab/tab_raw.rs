use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{QBox, QObject};
use qt_widgets::{QGridLayout, QPlainTextEdit, QWidget};

pub struct TabRaw {
	pub base: QBox<QWidget>,
}

impl StaticUpcast<QObject> for TabRaw {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl TabRaw {
	pub unsafe fn new() -> QBox<QWidget> {
		// Create main widget and add layout
		let base = QWidget::new_0a();
		let layout = QGridLayout::new_1a(&base);
		base.set_layout(&layout);
		// Create plain text field & add to layout
		let field = QPlainTextEdit::new();
		layout.add_widget_5a(&field, 0, 0, -1, -1);
		// Return base widget
		base
	}
}