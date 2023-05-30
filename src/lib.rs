use worker::*;
mod routes;
#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let router = Router::new();

    routes::register(router).run(req, env).await
}
