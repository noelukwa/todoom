use worker::Router;

mod notes;
mod todos;

pub fn register<'a>(router: Router<'a, ()>) -> Router<'a, ()> {
    router
        .get_async("/notes", notes::fetch_notes::handle)
        .post_async("/notes", notes::add_note::handle)
        .get_async("/todos", todos::fetch_todos::handle)
        .post_async("/todos", todos::add_todo::handle)
}
