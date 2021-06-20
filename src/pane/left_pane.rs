use crate::tab::tab_raw::TabRaw;
use crate::embedded_images::{ICON, get_icon};
use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
	qs, ContextMenuPolicy,
	QBox, QObject
};
use qt_widgets::{
	QGridLayout, QGroupBox, QPushButton, QTabWidget
};
use crate::tab::tab_encoded::TabEncoded;

pub struct LeftPane {
	pub base:			QBox<QGroupBox>,		// Base widget - a group box
	pub layout:			QBox<QGridLayout>,		// Group box layout
	pub tab_widget:		QBox<QTabWidget>,		// Tab container
	pub add_tab_btn:	QBox<QPushButton>,		// Tab bar "+" button
}

impl StaticUpcast<QObject> for LeftPane {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl LeftPane {
	pub unsafe fn new() -> Rc<Self> {
		// Base group box and layout
		let group_box = QGroupBox::from_q_string(&qs("Input Code(s)"));
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
			tab_widget: tabs,
			add_tab_btn: add_btn
		});
		// Finish initialization of pane & widgets
		this.initialize();
		this
	}
	/// Add widgets, connect slots
	unsafe fn initialize(self: &Rc<Self>) {
		// Show X buttons on tabs
		//&self.tab_widget.set_tabs_closable(true);
		// Set the tab position to the bottom (aka "South")
		//&self.tab_widget.set_tab_position(qt_widgets::q_tab_widget::TabPosition::South);
		// Set add_tab_btn to the empty corner, like a normal "Add" button
		&self.tab_widget.set_corner_widget_1a(&self.add_tab_btn);
		// Create a custom context menu for the add button
		&self.add_tab_btn.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
		&self.add_tab_btn.custom_context_menu_requested()
			.connect(&self.slot_add_tab_button_context_menu_requested());
		// Create a custom context menu for the tab bar
		&self.tab_widget.tab_bar().set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
		&self.tab_widget.tab_bar().custom_context_menu_requested()
			.connect(&self.slot_tab_context_menu_requested());
		// Connect clicked() slot
		&self.add_tab_btn.clicked()
			.connect(&self.slot_add_tab_clicked());
		// Add a Raw Code tab to start with
		let tab = TabRaw::new();
		// Connect this tab's field's text_changed slot
		tab.field.text_changed().connect(&self.slot_raw_text_changed());
		// Add this tab's base widget to the tab bar
		&self.tab_widget.add_tab_2a(&tab.base, &qs("Raw"));
		// Add tab control to pane's base widget
		&self.layout.add_widget_5a(&self.tab_widget, 0, 0, 1, -1);
	}

}