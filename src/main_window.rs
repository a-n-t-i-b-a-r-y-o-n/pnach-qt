use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject, QPtr};
use qt_widgets::{QMainWindow, QMenu, QMenuBar, QHBoxLayout, QPushButton, QWidget};
use crate::pane::{LeftPane, RightPane};
use crate::embedded_images::*;

pub struct MainWindow {
	pub window:		QBox<QMainWindow>,
	pub menu:		QBox<QMenuBar>,
	pub central:	QBox<QWidget>,
	pub layout:		QBox<QHBoxLayout>,
	pub left_pane:	Rc<LeftPane>,
	pub right_pane:	Rc<RightPane>,
	pub refresh_btn: QBox<QPushButton>,
}

impl StaticUpcast<QObject> for MainWindow {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.central.as_ptr().static_upcast()
	}
}

impl MainWindow {
	/// MainWindow factory
	pub unsafe fn new() -> Rc<Self> {
		// Qt window & central widget
		let window = QMainWindow::new_0a();
		let central = QWidget::new_1a(&window);
		window.set_central_widget(&central);
		// Top menu bar
		let menu = window.menu_bar().into_q_box();
		// Basic grid layout
		let layout = QHBoxLayout::new_1a(&central);
		// Create left and right panes and add to layout
		// NOTE: The left and right panes *must* be created here.
		// 		 This ensures they have global static pointers, and allows slot connections to work.
		let left_pane = LeftPane::new();
		let right_pane = RightPane::new();
		// Output refresh button, displayed in the right pane
		// NOTE: This button is created as part of the main window to allow easily moving
		//		 data between the left & right panes and their child widgets, but is drawn
		//		 in the right pane's button row for logical UI consistency.
		let refresh_btn = QPushButton::from_q_icon_q_string(&get_icon(ICON::REFRESH), &qs(""));

		Rc::new(Self {
			window,
			menu,
			central,
			layout,
			left_pane,
			right_pane,
			refresh_btn,
		})
	}
	/// Draw the main window
	pub unsafe fn show(self: &Rc<Self>) { &self.window.show(); }
	/// Initialize the main window
	pub unsafe fn initialize(self: &Rc<Self>){
		// Initialize window frame
		&self.initialize_frame();
		// Create & initialize window menu bar
		&self.initialize_menu_bar();
		// Initialize and configure window layout
		&self.initialize_layout();
		// Connect the refresh button clicked() slot
		&self.refresh_btn.clicked().connect(&self.slot_generate_pnach());
	}
	/// Initialize the main window frame
	unsafe fn initialize_frame(self: &Rc<Self>) {
		// Set window size and resize central widget to match
		&self.window.resize_2a(900, 600);
		&self.central.resize_1a(&self.window.size());
		// Set window title
		&self.window.set_window_title(&qs("PNach-rs"));
	}
	/// Create the main window menu bar
	unsafe fn initialize_menu_bar(self: &Rc<Self>) {
		// Add QMenu widgets to the QMenuBar
		let file_menu: &QPtr<QMenu> = &self.menu.add_menu_q_string(&qs("File"));
		let about_menu: &QPtr<QMenu> = &self.menu.add_menu_q_string(&qs("About"));

		// "New File" menu option
		let file_new = file_menu.add_action_q_string(&qs("New PNach File..."));
		file_new.triggered().connect(&self.slot_new_file());
		// Separator
		file_menu.add_separator();
		// "Open File" menu option
		let file_open = file_menu.add_action_q_string(&qs("Open PNach File..."));
		file_open.triggered().connect(&self.slot_open_file());
		// Add QMenu widgets to the "Help" QMenu
		let _help_help = about_menu.add_action_q_string(&qs("Help"));
		about_menu.add_separator();

		let _help_about = about_menu.add_action_q_string(&qs("About"));
	}
	/// Fill the main window layout with widgets
	unsafe fn initialize_layout(self: &Rc<Self>) {
		// Add panes to layout
		&self.layout.add_widget(&self.left_pane.base);
		&self.layout.add_widget(&self.right_pane.base);
		// Set the grid spacing
		&self.layout.set_spacing(10);
		// Ensure that left and right panes "stretch" identically
		&self.layout.set_stretch(0, 1);
		&self.layout.set_stretch(1, 1);
		// Add the refresh button to the right pane
		&self.right_pane.layout.add_widget_5a(&self.refresh_btn, 0, 0, 1, 1);
	}
}