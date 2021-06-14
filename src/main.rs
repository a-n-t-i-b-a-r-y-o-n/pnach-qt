use crate::main_window::MainWindow;
use qt_widgets::QApplication;

mod main_window;
mod main_window_slots;
mod input_pane;
mod output_pane;
mod embedded_images;
mod input_tab_raw;

fn main() {
    QApplication::init(|app| unsafe {
		let window = MainWindow::new();
		window.initialize();
		window.show();
		// Start the app
		QApplication::exec()
	})
}
