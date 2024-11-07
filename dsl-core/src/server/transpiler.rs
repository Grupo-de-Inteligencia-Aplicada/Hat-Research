use axum::extract::State;
use tracing::debug;
use quick_xml::events::Event;

use super::AppState;

/// This function will convert a Blockly XML workspace into Hat source code
#[axum::debug_handler]
pub(super) async fn transpile_workspace_to_hat(State(_): State<AppState>, xml: String) -> String {
    let mut reader = quick_xml::Reader::from_str(&xml);
    let mut depth = 0;
    while let Ok(event) = reader.read_event() {
        match event {
            Event::Start(tag) => {
                depth += 1;
                if depth == 2 {
                    // New block tag
                    debug!("{tag:#?}");
                }
            },
            Event::End(tag) => {
                depth -= 1;
            },
            Event::Eof => break,
            _ => {},
        }
    }
    todo!()
}

/// This function will convert Hat source code into a Blockly XML workspace
pub(super) async fn transpile_hat_to_workspace(State(_): State<AppState>, src: String) -> String {
    todo!()
}
