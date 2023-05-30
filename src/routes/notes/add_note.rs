use worker::{Request, Response, Result, RouteContext};

pub async fn handle(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Hello from Cloudflare Workers!")
}
