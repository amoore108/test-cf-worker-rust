use serde_json::json;
use worker::*;

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", |req, ctx| async move {
            let coords = req.cf().coordinates().unwrap();
            let postcode = req.cf().postal_code().unwrap();
            let city = req.cf().city().unwrap();

            let region = req.cf().region().unwrap_or_else(|| "unknown region".into());

            let kv = ctx.kv("KVSTORE")?;

            let test_val = kv.get("prevkey").text().await?.unwrap();

            return Response::from_json(
                &json!({ "coords": coords, "region": region, "postcode": postcode, "city": city, "kv": test_val}),
            );
        })
        .run(req, env)
        .await
}
