use axum::extract::State;

use super::AppState;

/// This function will convert a Blockly XML workspace into Hat source code
#[axum::debug_handler]
pub(super) async fn transpile_workspace_to_hat(State(_): State<AppState>, xml: String) -> String {
    todo!()
}

/// This function will convert Hat source code into a Blockly XML workspace
pub(super) async fn transpile_hat_to_workspace(State(_): State<AppState>, src: String) -> String {
    todo!()
}
