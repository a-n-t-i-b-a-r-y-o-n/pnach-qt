use std::rc::Rc;
use qt_core::{
	qs, slot,
	QBox, QObject, QPoint,
	SlotNoArgs, SlotOfQObject,
};
use qt_widgets::{QMenu, QTabWidget, QWidget, SlotOfIntBool, SlotOfQPoint};
use cpp_core::Ref;
use crate::pane::left_pane;
use crate::tab::tab_raw::TabRaw;
use crate::tab::tab_encoded::TabEncoded;


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
		// Add menu options, as "raw ptr" for comparison later
		let raw = menu.add_action_q_string(&qs("Raw")).as_raw_ptr();
		let encoded = menu.add_action_q_string(&qs("Encoded")).as_raw_ptr();
		// Display menu and capture selection
		let selection = menu.exec_1a_mut(&global_pos);
		// Check if something was selected
		if !selection.is_null() {
			let selected = selection.as_raw_ptr();
			// Identify the option that was selected by comparing raw ptrs
			if selected == raw {
				// Create a tab for raw codes
				let tab = TabRaw::new();
				// Connect this tab's field's text_changed slot
				tab.field.text_changed().connect(&self.slot_raw_text_changed());
				// Add this tab's base widget to the tab bar
				&self.tab_widget.add_tab_2a(&tab.base, &qs("Raw"));
			}
			else if selected == encoded {
				// Create a new tab for encoded cheats
				let tab = TabEncoded::new();
				// Connect this tab's field's text_changed slot
				tab.field.text_changed().connect(&self.slot_encoded_text_changed());
				// Connect this tab's panel radio button slots
				tab.panel_radios.button_toggled2().connect(&self.slot_radio_toggled());
				// Add this tab's base widget to the tab bar
				&self.tab_widget.add_tab_2a(&tab.base, &qs("Encoded"));
			}
		}
	}

	#[slot(SlotOfQPoint)]
	pub unsafe fn add_tab_button_context_menu_requested(self: &Rc<Self>, _: Ref<QPoint>) { self.add_tab_clicked(); }

	#[slot(SlotOfQPoint)]
	pub unsafe fn tab_context_menu_requested(self: &Rc<Self>, pos: Ref<QPoint>) {
		// Only trigger if there's more than 1 tab
		if self.tab_widget.tab_bar().count() > 1 {
			// Translate given point to global point
			let global_pos = QWidget::map_to_global(&self.base, pos);
			// Create menu
			let menu = QMenu::new();
			// Add menu options, as "raw ptr" for comparison later
			let close = menu.add_action_q_string(&qs("Close Tab")).as_raw_ptr();
			let close_others = menu.add_action_q_string(&qs("Close Other Tabs")).as_raw_ptr();
			// Display menu and capture selection
			let selection = menu.exec_1a_mut(&global_pos);
			// Check if something was selected
			if !selection.is_null() {
				// Get the menu selection
				let menu_option = selection.as_raw_ptr();
				// Identify which tab was clicked
				let tab: i32 = self.tab_widget.tab_bar().tab_at(pos);
				// Identify if the current tab was the one clicked
				let current_selected = self.tab_widget.tab_bar().current_index() == tab;
				// Identify the option that was selected by comparing raw ptrs
				if menu_option == close {
					// Switch to the next tab if need be, or previous if there isn't a next
					if current_selected {
						if self.tab_widget.tab_bar().count() > tab {
							self.tab_widget.tab_bar().set_current_index(tab+1);
						} else {
							self.tab_widget.tab_bar().set_current_index(tab-1);
						}
					}
					// Close the selected tab
					&self.tab_widget.tab_bar().remove_tab(tab);
				}
				else if menu_option == close_others {
					// Move tab to the left side
					self.tab_widget.tab_bar().move_tab(tab, 0);
					// Select this unclosed tab
					self.tab_widget.tab_bar().set_current_index(0);
					// Remove every other tab
					for i in 1..self.tab_widget.tab_bar().count() {
							self.tab_widget.tab_bar().remove_tab(i);
					}
				}
			}
		}
	}

	#[slot(SlotNoArgs)]
	pub unsafe fn raw_text_changed(self: &Rc<Self>) {
		//println!("Text changed on tab {}.", self.tab_widget.current_index());
	}

	#[slot(SlotNoArgs)]
	pub unsafe fn encoded_text_changed(self: &Rc<Self>) {
		//println!("Text changed on tab {}.", self.tab_widget.current_index());
	}

	#[slot(SlotOfIntBool)]
	pub unsafe fn radio_toggled(self: &Rc<Self>, id: i32, checked: bool) {
		//println!("Radio {} toggled.", id);
	}
}