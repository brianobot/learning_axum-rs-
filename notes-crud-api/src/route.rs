use std::sync::Arc;

use crate::{
    AppState,
    handler::{
        create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
        health_checker_handler, list_notes_handler,
    },
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/notes",
            get(list_notes_handler).post(create_note_handler),
        )
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .with_state(app_state)
}
