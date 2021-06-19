use crate::main_window;
use std::rc::Rc;
use qt_core::{
	qs, slot, ContextMenuPolicy,
	QBox, QObject, QPoint,
	SlotNoArgs
};
use qt_widgets::{QApplication, QMenu, QMessageBox, QWidget, SlotOfQPoint};
use cpp_core::{NullPtr, Ref};
use crate::pane::left_pane;


/// Left pane slots
impl left_pane::LeftPane {
	#[slot(SlotNoArgs)]
	/// The Add Tab "+" button opens a context menu to allow selecting different types of input tabs
	pub unsafe fn add_tab_clicked(self: &Rc<Self>) {
		// Get the global position of the button on screen
		// TODO: Both of these are broken on my install, but the following selection works in fullscreen (ish).
		// TODO: Test global_pos implementation
		let global_pos = QWidget::map_to_global(&self.base, &self.add_tab_btn.pos());	// = (192, 417)
		//let global_pos = self.add_tab_btn.map_to_global(&self.add_tab_btn.pos());		// = (388, 832)
		//println!("({:?}, {:?})", global_pos.x(), global_pos.y());
		// Create menu
		let menu = QMenu::new();
		let raw = menu.add_action_q_string(&qs("Raw")).as_raw_ptr();
		let encoded = menu.add_action_q_string(&qs("Encoded")).as_raw_ptr();
		let detect = menu.add_action_q_string(&qs("Auto-Detect..."));

		let selection = menu.exec_1a_mut(&global_pos);
		if !selection.is_null() {
			let selected = selection.as_raw_ptr();
			if selected == raw {
				println!("Raw selected.");
			}
			else if selected == encoded {
				println!("Encoded selected.");
			}
		}
		else {
			println!("Nothing selected.");
		}

		//println!("{:?}", selection);
	}

	#[slot(SlotOfQPoint)]
	pub unsafe fn add_tab_context_menu_requested(self: &Rc<Self>, pos: Ref<QPoint>) { self.add_tab_clicked(); }

}