use tide::{Request, Response, Next, Error, log};
use std::future::Future;
use std::pin::Pin;

pub async fn log_middleware<'a>(req: Request<()>, next: Next<'a, ()>) -> Result<Response, Error> {
    log::info!("Incoming request: {} {}", req.method(), req.url());
    let res = next.run(req).await;
    log::info!("Response status: {}", res.status());
    Ok(res)
}

pub fn boxed_log_middleware<'a>(req: Request<()>, next: Next<'a, ()>) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'a>> {
    Box::pin(log_middleware(req, next))
}