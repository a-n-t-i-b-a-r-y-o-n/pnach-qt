use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs};
use qt_widgets::{QButtonGroup, QFormLayout, QGridLayout, QPlainTextEdit, QPushButton, QRadioButton, QWidget};
use std::convert::TryInto;

pub struct TabEncoded {
	pub base:			QBox<QWidget>,			// Central widget in tab
	pub layout:			QBox<QGridLayout>,		// Layout of central widget
	pub field:			QBox<QPlainTextEdit>,	// Input field for encoded cheats
	pub panel:			QBox<QWidget>,			// Side panel within this tab
	pub panel_layout:	QBox<QGridLayout>,		// Layout of side panel
	pub panel_radios:	QBox<QButtonGroup>,		// QButtonGroup of the panel's radio buttons
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
		// Create button group for radio buttons
		let panel_radios = QButtonGroup::new_1a(&panel_layout);

		let this = Rc::new(Self {
			base,
			layout,
			field,
			panel,
			panel_layout,
			panel_radios,
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
		let supported_encodings = vec![
			"ActionReplay v1",
			"ActionReplay v2",
			"ActionReplay MAX",
			"CodeBreaker V1+",
			"CodeBreaker V7",
			"GameShark V1",
			"GameShark V2",
			"GameShark V3+",
			"GameShark V5+",
			"Swap Magic",
			"Xploder V1-3",
			"Xploder V4",
			"Xploder V5"
		];
		for (index, name) in supported_encodings.into_iter().enumerate() {
			let i = index.try_into().unwrap();
			// Create radio buttons for each type of encoding supported by OmniConvert
			let button = QRadioButton::from_q_string(&qs(name));
			// Add these radio buttons to the panel button group
			&self.panel_radios.add_button_2a(&button, i);
			// Put the radio buttons in the panel
			&self.panel_layout.add_widget_5a(&button, i, 0, 1, -1);
		}

	}

}