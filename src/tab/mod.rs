use std::rc::Rc;
use qt_core::{qs, QVariant, QPtr};
use qt_widgets::{QPlainTextEdit, QRadioButton, QWidget};
use cpp_core::CppBox;
use std::any::Any;

pub mod tab_raw;
pub mod tab_encoded;

#[derive(Debug)]
pub enum TabType { RAW, ENCODED }

/// Heuristic for identifying tabs by child count
unsafe fn identify_tab_type(tab: &QPtr<QWidget>) -> TabType {
	/*
	Current known child counts:
		1:	RAW
		2:	ENCODED
	 */
	// Return deduced type, assume RAW if unknown
	match tab.layout().count() {
		2 => TabType::ENCODED,
		_ => TabType::RAW,
	}
}

/// Get raw and/or decoded codes from a tab
pub unsafe fn get_raw_codes(tab: &QPtr<QWidget>) -> String {
	match identify_tab_type(tab) {
		TabType::RAW => {
			// Get input field contents
			tab.layout().item_at(0).widget().static_downcast::<QPlainTextEdit>().to_plain_text().to_std_string()
		}
		TabType::ENCODED => {
			// Get encoded cheats string
			let cheats = tab.layout().item_at(0).widget().static_downcast::<QPlainTextEdit>().to_plain_text().to_std_string();
			// Get handle on the tab panel
			let panel = &tab.layout().item_at(1).widget();
			// Selected encoding button id
			let mut encoding = 0;
			// Iterate panel radio button to identify which one is checked
			for i in 0..panel.layout().count() {
				// Get handle on radio button
				let radio = panel.layout().item_at(i).widget().static_downcast::<QRadioButton>();
				// Identify if it's checked
				if radio.is_checked() {
					// Get this radio button's id from its button group
					encoding = radio.group().id(&radio);
				}
			}
			decode(&cheats, encoding)
		}
	}
}

/// Detect possible code type
fn decode(encoded_cheats: &str, encoding: i32) -> String {
	String::from(encoded_cheats)
}

