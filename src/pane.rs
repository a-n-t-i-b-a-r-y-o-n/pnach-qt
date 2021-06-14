/// General pane operations
pub trait PaneOperations {
	unsafe fn update_text(new_text: &str) { }
	unsafe fn append_text(new_text: &str) { }
}