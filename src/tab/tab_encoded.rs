use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs};
use qt_widgets::{QButtonGroup, QFormLayout, QGridLayout, QPlainTextEdit, QPushButton, QRadioButton, QWidget};

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
		// Create radio buttons for each type of encoding supported by OmniConvert
		let ARv1 =	QRadioButton::from_q_string(&qs("ActionReplay v1"));
		let ARv2 =	QRadioButton::from_q_string(&qs("ActionReplay v2"));
		let ARMax =	QRadioButton::from_q_string(&qs("ActionReplay MAX"));
		let CB1 =	QRadioButton::from_q_string(&qs("CodeBreaker V1+"));
		let CB7 =	QRadioButton::from_q_string(&qs("CodeBreaker V7"));
		let GS1 =	QRadioButton::from_q_string(&qs("GameShark V1"));
		let GS2 =	QRadioButton::from_q_string(&qs("GameShark V2"));
		let GS3 =	QRadioButton::from_q_string(&qs("GameShark V3+"));
		let GS5 =	QRadioButton::from_q_string(&qs("GameShark V5+"));
		let XP1 =	QRadioButton::from_q_string(&qs("Xploder V1-3"));
		let XP4 =	QRadioButton::from_q_string(&qs("Xploder V4"));
		let XP5 =	QRadioButton::from_q_string(&qs("Xploder V5"));
		let SM =	QRadioButton::from_q_string(&qs("Swap Magic"));
		// Add these radio buttons to the panel button group
		&self.panel_radios.add_button_2a(&ARv1, 0);
		&self.panel_radios.add_button_2a(&ARv2, 1);
		&self.panel_radios.add_button_2a(&ARMax, 2);
		&self.panel_radios.add_button_2a(&CB1, 3);
		&self.panel_radios.add_button_2a(&CB7, 4);
		&self.panel_radios.add_button_2a(&GS1, 5);
		&self.panel_radios.add_button_2a(&GS2, 6);
		&self.panel_radios.add_button_2a(&GS3, 7);
		&self.panel_radios.add_button_2a(&GS5, 8);
		&self.panel_radios.add_button_2a(&XP1, 9);
		&self.panel_radios.add_button_2a(&XP4, 10);
		&self.panel_radios.add_button_2a(&XP5, 11);
		&self.panel_radios.add_button_2a(&SM, 12);
		// Put the radio buttons in the panel
		&self.panel_layout.add_widget_5a(&ARv1, 0, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&ARv2, 1, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&ARMax, 2, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&CB1, 3, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&CB7, 4, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&GS1, 5, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&GS2, 6, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&GS3, 7, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&GS5, 8, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&XP1, 9, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&XP4, 10, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&XP5, 11, 0, 1, -1);
		&self.panel_layout.add_widget_5a(&SM, 12, 0, 1, -1);
	}

}