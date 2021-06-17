pub mod left_pane;
pub mod right_pane;
pub mod left_pane_slots;

use crate::main_window;
use std::rc::Rc;
use qt_core::{
	qs, slot, ContextMenuPolicy,
	QBox, QObject, QPoint,
	SlotNoArgs
};
use qt_widgets::{QApplication, QMenu, QMessageBox, SlotOfQPoint};
use cpp_core::{NullPtr, Ref};

impl left_pane::LeftPane {
	// ## Slots
	#[slot(SlotNoArgs)]
	pub unsafe fn button_clicked(self: &Rc<Self>) {
		//println!("Clicked!");
		//&self.field.set_plain_text(&qs("Clicked!"));
		let m = QMessageBox::information_q_widget2_q_string(NullPtr, &qs("Example"), &qs("MessageBox"));
		println!("{:?}", m);
	}

	#[slot(SlotOfQPoint)]
	pub unsafe fn on_add_tab_btn_context_menu_requested(self: &Rc<Self>, pos: Ref<QPoint>) {
		// Get the global position of the provided point (which is relative to the button)
		//let global_position = self.tabs.viewport().map_to_global(pos);
		let menu = QMenu::new();
		menu.add_action_q_string(&qs("Test 1"));

		let selected = menu.exec_1a_mut(pos);

	}
}