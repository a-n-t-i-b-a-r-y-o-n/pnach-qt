use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, QBox, QObject, SlotNoArgs};
use qt_widgets::{QGridLayout, QGroupBox, QPushButton, QTabWidget};
use crate::input_tab_raw::InputTab_Raw;
use crate::embedded_images::{ICON, get_icon};

pub struct InputPane {
	pub base:			QBox<QGroupBox>,		// Base widget - a group box
	pub layout:			QBox<QGridLayout>,		// Group box layout
	pub tabs:			QBox<QTabWidget>,
	pub add_tab_btn:	QBox<QPushButton>,
}

impl StaticUpcast<QObject> for InputPane {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl InputPane {
	pub unsafe fn new() -> Rc<Self> {
		// Base group box and layout
		let group_box = QGroupBox::from_q_string(&qs("Input Codes"));
		let layout = QGridLayout::new_1a(&group_box);
		group_box.set_layout(&layout);
		// Tab bar
		let tabs = QTabWidget::new_1a(&group_box);
		// New tab button
		let add_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::ADD), &qs(""));
		// Compose pane object w/ above widgets
		let this = Rc::new(Self {
			base: group_box,
			layout,
			tabs,
			add_tab_btn: add_btn
		});
		// Finish initialization of pane & widgets
		this.initialize_pane();
		this
	}
	/// Add widgets, connect slots
	unsafe fn initialize_pane(self: &Rc<Self>) {
		// Set the tab position to the bottom (aka "South")
		&self.tabs.set_tab_position(qt_widgets::q_tab_widget::TabPosition::South);
		// Add tab control to pane's base widget
		&self.layout.add_widget_5a(&self.tabs, 0, 0, 1, -1);
		// Set add_tab_btn to the empty corner, like a normal "Add" button
		&self.tabs.set_corner_widget_1a(&self.add_tab_btn);
		// Add a Raw Code tab to start with
		let tab1 = InputTab_Raw::new();
		self.tabs.add_tab_2a(&tab1.base, &qs("RAW"));
	}

	// ## Slots
	#[slot(SlotNoArgs)]
	pub unsafe fn button_clicked(self: &Rc<Self>) {
		println!("Clicked!");
		//&self.field.set_plain_text(&qs("Clicked!"));
	}
}