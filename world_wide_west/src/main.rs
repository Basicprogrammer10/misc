use afire::{trace, trace::Level, Content, Method, Middleware, Response, Server};
use app::App;
use path_normalizer::PathNormalizer;

use crate::database::Database;

mod app;
mod completer;
mod database;
mod path_normalizer;

fn main() {
    trace::set_log_level(Level::Trace);
    let mut server = Server::new("localhost", 8080)
        .state(App::new())
        .keep_alive(false);
    PathNormalizer.attach(&mut server);

    server.stateful_route(Method::ANY, "**", |app, req| {
        let force_regen = req.query.has("r");
        let db = app.db();
        let completion = match db.get_completion(&req.path) {
            Some(i) if !force_regen => i,
            _ => {
                println!("[*] Loading completion for `{}`", req.path);
                let out = app.completer.complete(req).unwrap();
                db.set_completion(&req.path, &out);
                out
            }
        };
        drop(db);

        Response::new()
            .content(Content::Custom(&completion.content_type))
            .header("X-Tokens-Used", completion.tokens.to_string())
            .bytes(&completion.body)
    });

    server.start_threaded(8).unwrap();
}
