use std::rc::Rc;
use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject, QPtr};
use qt_widgets::{QDialog, QMainWindow, QMenu, QMenuBar, QGroupBox, QHBoxLayout, QWidget};
use crate::pane::{left_pane::LeftPane, right_pane::RightPane};

pub struct MainWindow {
	pub window:		QBox<QMainWindow>,
	pub menu:		QBox<QMenuBar>,
	pub central:	QBox<QWidget>,
	pub layout:		QBox<QHBoxLayout>,
	pub left_pane:	Rc<LeftPane>,
	pub right_pane:	Rc<RightPane>,
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
		// NOTE: The left and right panes have to be created here.
		// 		 This ensures they have global static pointers, and allows slot connections to work.
		let left_pane = LeftPane::new();
		let right_pane = RightPane::new();
		layout.add_widget(&left_pane.base);
		layout.add_widget(&right_pane.base);

		Rc::new(Self {
			window,
			menu,
			central,
			layout,
			left_pane,
			right_pane,
		})
	}
	/// Draw the main window
	pub unsafe fn show(self: &Rc<Self>) { &self.window.show(); }
	/// Initialize the main window
	pub unsafe fn initialize(self: &Rc<Self>){
		&self.initialize_frame();
		&self.initialize_menu_bar();
		&self.initialize_layout();
	}
	/// Initialize the main window frame
	unsafe fn initialize_frame(self: &Rc<Self>) {
		// Set window size
		&self.window.resize_2a(500, 500);
		&self.central.resize_1a(&self.window.size());
		// Set window title
		&self.window.set_window_title(&qs("PNach-rs"));
	}
	/// Create the main window menu bar
	unsafe fn initialize_menu_bar(self: &Rc<Self>) {
		// Add QMenu widgets to the QMenuBar
		let file_menu: &QPtr<QMenu> = &self.menu.add_menu_q_string(&qs("File"));
		let about_menu: &QPtr<QMenu> = &self.menu.add_menu_q_string(&qs("About"));

		// Add QMenu widgets to the "File" QMenu, along with connectors
		let file_new = file_menu.add_action_q_string(&qs("New PNach File..."));
		file_new.triggered().connect(&self.slot_new_file());
		file_menu.add_separator();

		let _file_open = file_menu.add_action_q_string(&qs("Open PNach File..."));

		// Add QMenu widgets to the "Help" QMenu
		let _help_help = about_menu.add_action_q_string(&qs("Help"));
		about_menu.add_separator();

		let _help_about = about_menu.add_action_q_string(&qs("About"));
	}
	/// Fill the main window layout with widgets
	unsafe fn initialize_layout(self: &Rc<Self>) {
		// Set the grid spacing
		&self.layout.set_spacing(10);

		// Ensure that left and right panes "stretch" identically
		&self.layout.set_stretch(0, 1);
		&self.layout.set_stretch(1, 1);

	}
}