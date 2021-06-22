use crate::main_window;
use crate::tab;
use libpnach::*;
use std::rc::Rc;
use qt_core::{qs, slot, QString, SlotNoArgs};
use qt_widgets::QMessageBox;

impl main_window::MainWindow {
	/// Start over with a clean slate in a new file
	#[slot(SlotNoArgs)]
	pub unsafe fn new_file(self: &Rc<Self>) {
		QMessageBox::information_q_widget2_q_string(&self.central, &qs("Example"), &qs("MessageBox"));
	}

	/// Right pane refresh button clicked()
	#[slot(SlotNoArgs)]
	pub unsafe fn refresh_clicked(self: &Rc<Self>) {
		// Clear output field contents
		&self.right_pane.field.clear();
		// Create a new PNach object
		let mut pnach = pnach_file::PNachFile::new(
			&self.left_pane.game_name.text().to_std_string(),
			&self.left_pane.game_crc.text().to_std_string()
		);
		// Iterate left pane tabs
		for i in 0..self.left_pane.tab_widget.tab_bar().count() {
			// Get main widget holding the content of the tab at this index
			let content = &self.left_pane.tab_widget.widget(i);
			// Decode and extract codes from this tab
			tab::get_raw_codes(content).into_iter()
				.for_each(|c| {
					pnach.codes.push(c)
				});
		}

		&self.right_pane.field.set_plain_text(&qs(pnach.to_string()));
	}
}