use crate::tab::{TabRaw, TabEncoded};
use crate::embedded_images::{ICON, get_icon};
use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
	qs, ContextMenuPolicy,
	QBox, QObject
};
use qt_widgets::{
	QFormLayout, QGridLayout, QGroupBox,
	QLabel, QLineEdit, QPushButton,
	QTabWidget, QWidget
};

pub struct LeftPane {
	pub base:			QBox<QWidget>,			// Base widget - a group box
	pub layout:			QBox<QGridLayout>,		// Base widget layout
	pub top_box:		QBox<QGroupBox>,		// Top box
	pub top_layout:		QBox<QGridLayout>,		// Top box layout
	pub tab_widget:		QBox<QTabWidget>,		// Tab container
	pub add_tab_btn:	QBox<QPushButton>,		// Tab bar "+" button
	pub bottom_box:		QBox<QGroupBox>,
	pub bottom_layout:	QBox<QFormLayout>,
	pub game_name:		QBox<QLineEdit>,
	pub game_crc:		QBox<QLineEdit>,
}

impl StaticUpcast<QObject> for LeftPane {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.base.as_ptr().static_upcast()
	}
}

impl LeftPane {
	pub unsafe fn new() -> Rc<Self> {
		// Base central widget
		let base = QWidget::new_0a();
		let layout = QGridLayout::new_1a(&base);
		base.set_layout(&layout);
		// Top group box and layout
		let top_box = QGroupBox::from_q_string(&qs("Input Code(s)"));
		let top_layout = QGridLayout::new_1a(&top_box);
		top_box.set_layout(&top_layout);
		// Tab bar in top box
		let tab_widget = QTabWidget::new_1a(&top_box);
		// New tab button in tab bar
		let add_tab_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::ADD), &qs(""));
		// Bottom group box and layout
		let bottom_box = QGroupBox::from_q_string(&qs("Game Info"));
		let bottom_layout = QFormLayout::new_1a(&bottom_box);
		bottom_box.set_layout(&bottom_layout);
		// Game name field
		let game_name = QLineEdit::new();
		// Game CRC field
		let game_crc = QLineEdit::new();
		// Compose pane object w/ above widgets
		let mut this = Rc::new(Self {
			base,
			layout,
			top_box,
			top_layout,
			tab_widget,
			add_tab_btn,
			bottom_box,
			bottom_layout,
			game_name,
			game_crc
		});
		// Finish initialization of pane & widgets
		this.initialize();
		this
	}
	/// Add widgets, connect slots
	unsafe fn initialize(self: &Rc<Self>) {
		self.initialize_top();
		self.initialize_bottom();
		self.initialize_slots();
		self.initialize_layouts();
	}

	unsafe fn initialize_bottom(self: &Rc<Self>) {
		// Show X buttons on tabs
		//&self.tab_widget.set_tabs_closable(true);
		// Set the tab position to the bottom (aka "South")
		//&self.tab_widget.set_tab_position(qt_widgets::q_tab_widget::TabPosition::South);
		// Set add_tab_btn to the empty corner, like a normal "Add" button
		&self.tab_widget.set_corner_widget_1a(&self.add_tab_btn);
		// Add one of each tab to start with
		let raw = TabRaw::new();
		let encoded	= TabEncoded::new();
		// Connect these tabs' fields' text_changed slots
		raw.field.text_changed().connect(&self.slot_left_pane_text_changed());
		encoded.field.text_changed().connect(&self.slot_left_pane_text_changed());
		// Add these tabs' base widgets to the tab bar
		&self.tab_widget.add_tab_2a(&raw.base, &qs("Raw"));
		&self.tab_widget.add_tab_2a(&encoded.base, &qs("Encoded"));
	}

	unsafe fn initialize_top(self: &Rc<Self>) {
		// Field label
		&self.bottom_layout.add_widget(&QLabel::from_q_string(&qs("Game name:")));
		// Add the "Game Name" field
		&self.bottom_layout.add_widget(&self.game_name);
		// Field label
		&self.bottom_layout.add_widget(&QLabel::from_q_string(&qs("Game CRC:")));
		// Add the "Game CRC" field
		&self.bottom_layout.add_widget(&self.game_crc);
	}

	unsafe fn initialize_layouts(self: &Rc<Self>) {
		// Set the base layout margin to 0 (otherwise there's padding space on all sides)
		&self.layout.set_margin(0);
		// Add tab control to top pane's base widget
		&self.top_layout.add_widget_5a(&self.tab_widget, 0, 0, 1, -1);
		// Add top box to left pane layout
		&self.layout.add_widget_5a(&self.top_box, 1, 0, 1, 1);
		// Add bottom box to left pane layout
		&self.layout.add_widget_5a(&self.bottom_box, 0, 0, 1, 1);
	}

	unsafe fn initialize_slots(self: &Rc<Self>) {
		// Create a custom context menu for the add button
		&self.add_tab_btn.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
		&self.add_tab_btn.custom_context_menu_requested()
			.connect(&self.slot_add_tab_button_context_menu_requested());
		// Create a custom context menu for the tab bar
		&self.tab_widget.tab_bar().set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
		&self.tab_widget.tab_bar().custom_context_menu_requested()
			.connect(&self.slot_tab_context_menu_requested());
		// Connect clicked() slot to new tab button
		&self.add_tab_btn.clicked()
			.connect(&self.slot_add_tab_clicked());
		// text_changed() slots for game info fields
		&self.game_name.text_changed()
			.connect(&self.slot_left_pane_text_changed());
		&self.game_crc.text_changed()
			.connect(&self.slot_left_pane_text_changed());
	}

}