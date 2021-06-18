use crate::main_window::MainWindow;
use qt_widgets::QApplication;

mod main_window;
mod main_window_slots;
mod embedded_images;
mod pane;
mod tab;
mod assimilation;
mod sanitization;

fn main() {
    QApplication::init(|app| unsafe {
		let window = MainWindow::new();
		window.initialize();
		window.show();
		// Start the app
		QApplication::exec()
	})
}
