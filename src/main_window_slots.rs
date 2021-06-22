use crate::main_window;
use libpnach::*;
use std::rc::Rc;
use qt_core::{qs, slot, QString, SlotNoArgs};
use qt_widgets::{QFileDialog, QMessageBox, q_file_dialog::AcceptMode};
use crate::pane::{LeftPane, RightPane};
use crate::tab::{self, TabRaw, TabEncoded};

impl main_window::MainWindow {
	/// Start over with a clean slate in a new file
	#[slot(SlotNoArgs)]
	pub unsafe fn new_file(self: &Rc<Self>) {
		// Clear game info fields
		&self.left_pane.game_name.clear();
		&self.left_pane.game_crc.clear();
		// Clear output field
		&self.right_pane.field.clear();
		// Iterate left pane tabs and remove them
		for x in 0..self.left_pane.tab_widget.tab_bar().count() {
			self.left_pane.tab_widget.remove_tab(0);
		}
		// Create a "raw" and "encoded" tab to add to the tab bar like normal
		let raw = TabRaw::new();
		let encoded	= TabEncoded::new();
		// Connect these tabs' fields' text_changed slots
		raw.field.text_changed().connect(&self.left_pane.slot_left_pane_text_changed());
		encoded.field.text_changed().connect(&self.left_pane.slot_left_pane_text_changed());
		// Add these tabs' base widgets to the tab bar
		&self.left_pane.tab_widget.add_tab_2a(&raw.base, &qs("Raw"));
		&self.left_pane.tab_widget.add_tab_2a(&encoded.base, &qs("Encoded"));
	}

	/// Open existing PNach file
	#[slot(SlotNoArgs)]
	pub unsafe fn open_file(self: &Rc<Self>) {
		// Create the file picker dialog
		let open_dialog = QFileDialog::from_q_widget_q_string(&self.central, &qs("Open PNach File..."));
		// Set the dialog to open mode (the default per the docs)
		open_dialog.set_accept_mode(AcceptMode::AcceptOpen);
		// Show dialog
		open_dialog.show();
	}

	/// Right pane refresh button clicked()
	#[slot(SlotNoArgs)]
	pub unsafe fn generate_pnach(self: &Rc<Self>) {
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
		// Pretty-print the PNach file in the box
		&self.right_pane.field.set_plain_text(&qs(pnach.to_string()));
	}
}