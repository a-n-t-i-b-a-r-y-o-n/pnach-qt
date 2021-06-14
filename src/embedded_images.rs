use qt_core::QByteArray;
use qt_gui::{QIcon, QPixmap};
use cpp_core::CppBox;

pub enum ICON {
	ADD,
	COPY,
	PASTE,
	REFRESH,
	SAVE
}
/// Return embedded QIcons
pub unsafe fn get_icon(icon: ICON) -> CppBox<QIcon> {
	// TODO: Add separate light icon theme w/ match {} and new arg
	// TODO: Store icon assets separately or something
	// Current scheme: Material Theme icons - black
	let data = match icon {
		ICON::ADD =>		"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAQAAAD9CzEMAAAANElEQVRYw+3SwQkAQAgDQftvWj9XwSEiOJsC5pMISdfKNwAAAJgA8nOAPsBNAQDAZkDSzgo1fM8xWMDdPAAAAABJRU5ErkJggg==",
		ICON::COPY => 		"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAQAAAD9CzEMAAAAd0lEQVRYw+3XMQ7AIAwDQP+D9+dpMPEHukdtFVswtLKZkxtQpAT4UxoCA4t8RPtJN6eAkNoTQD8N0AVsDAgAPxcDgVYF1LmYT0QG1LlYiBrQZaDXAOXTX2sMGDBgwIABA1KNAQMGDiy/OdvX95ztB8gdsfWE+moubYmaqr7mfyEAAAAASUVORK5CYII=",
		ICON::PASTE =>		"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAQAAAD9CzEMAAAAzElEQVRYw2NgIB6EM2xmuM1wnWE9gzMD1QEPw36G/0hwOQMLNY1nYTiMYjzECiqCZAzjQZCsgJJhmM1wGatxIHiYwYHBG4/8eYZ+BgF8xiswvMepGQR1wKq88aq5j8+K9Xi1/gc6AARsCKjqx23BewJa1wOt0MES4egBhRPAlKDzCUFc+kctGLWAdDBqwagFoxaMWjBqwagFoxaMWjBqwWCx4DPRjUV88DpuC3ZTxYLZuC0wYPhNsfHvgT1VBnxW7KYgoN6De3GjYJABAP3gmm3i5GmKAAAAAElFTkSuQmCC",
		ICON::REFRESH =>	"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAQAAAD9CzEMAAABrElEQVRYw+WYPYoCQRCFHyr+gJOPYCiGZmbGiph6BD2HMKGZYnsQLzChHkEwEcVcVCatDba2adafrkYLFrbMxlfv656pfiMC/60m2vakbU/a9qRtT9r2pG1PuGKHFAZdFDXs3c8FBrEmgEC4IUFJF0FYv78PF1FFAwMYHJxrR7Q+P6Z5jHByELHGQYuwcm5UwLNoi6Mih7lFJFL7OrInqfko7HJ2FzfUZIAlCPQUcV+RfRZLiX0FZ5Z3xLd0zB1nyekesniPghiQx5G7en7xjKXToBlbcJfxSzfytTjV567UL/3ZbD0I0OSurV+asbQcBIhswnrrwtIoCFDmrswv3bK0GQSo20zyVsrSfhCgx10bv9SwdBEEmHLXTL6WI/Ji+wL23DX0i4s2KsZiQMdGRUUedoTTw0mavIjypWw9NdwYsULu7tvnOZvJD2diXyLzO8TzKG/LZ6KEtUWsft0oeoEIqNhmEuGEkTNR9ClEy0EQDjAYoIGqc+0Du1h7fnK9jSghsROlhABiGJuwKoDv092FQYodrhr2eDCmavbQtoe2PbTtoW2v/mfC36sv2dpoZFRPkDsAAAAASUVORK5CYII=",
		ICON::SAVE =>		"iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAQAAAD9CzEMAAAAx0lEQVRYw+3XwQ2DMAwF0H/KEnQPpoIVOg2sAkeWgFMrVTAAHFAEVFFlO3YlJH9fkZ8UnCgBPB7tPNBiwsqsJ739m92cRbTC9mRizABIRPyUkxIfOiEBWMQZoCwKm5ACZEIOXAkTACitATigDEjigC0QUKPDggUdagRtoMBwmZ8BhSYQvtrvRNADquQuqPSAPgn0esCcBOYbAeZLZP6TzcfUfKP94ai4y2mad/mNNdpc349qLB4gR72S85X5hIo1ofnd3uORZAM0IOJix4EjxwAAAABJRU5ErkJggg==",
	};
	// Create empty QPixmap
	let pixmap1 = QPixmap::new();
	// Create QByteArray from base64 encoded string slice
	pixmap1.load_from_data_q_byte_array(&QByteArray::from_base64_1a(&QByteArray::from_slice(data.as_bytes())));
	// Create QIcon from QPixmap
	QIcon::from_q_pixmap(&pixmap1)
}