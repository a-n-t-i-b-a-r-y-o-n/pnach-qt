use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject};
use qt_widgets::{QFormLayout, QGridLayout, QPlainTextEdit, QPushButton, QWidget};

pub struct TabEncoded {
	pub base:			QBox<QWidget>,			// Central widget in tab
	pub layout:			QBox<QGridLayout>,		// Layout of central widget
	pub field:		QBox<QPlainTextEdit>,	// Input field for encoded cheats
	pub panel:			QBox<QWidget>,			// Side panel within this tab
	pub panel_layout:	QBox<QGridLayout>,		// Layout of side panel
}

impl StaticUpcast<QObject> for TabEncoded {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl TabEncoded {
	pub unsafe fn new() -> Rc<Self> {
		// Create base widget & layout
		let base = QWidget::new_0a();
		let layout = QGridLayout::new_1a(&base);
		base.set_layout(&layout);
		// Create text fields
		let field = QPlainTextEdit::new();
		// Create side panel
		let panel = QWidget::new_0a();
		let panel_layout = QGridLayout::new_1a(&panel);
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
	/// Add fields & panel to base layout
	unsafe fn initialize_tab(self: &Rc<Self>) {
		&self.layout.add_widget_5a(&self.field, 0, 0, 1, 1);
		&self.layout.add_widget_5a(&self.panel, 0, 1, -1, 1);
	}

	unsafe fn initialize_panel(self: &Rc<Self>) {
		let button = QPushButton::from_q_string(&qs("Decode"));
		&self.panel_layout.add_widget_5a(&button, 0, 1, 1, 1);
	}

}