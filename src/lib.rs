use leptos::*;
use worker::*;

#[component]
pub fn SimpleCounter(cx: Scope, req: Request) -> impl IntoView {
    let coords = req.cf().coordinates().unwrap();
    let postcode = req.cf().postal_code().unwrap();
    let city = req.cf().city().unwrap();
    let region = req.cf().region().unwrap_or_else(|| "unknown region".into());

    view! { cx,
        <div>
            <table>
            <tr>
              <th>"Coords"</th>
              <th>"Postcode"</th>
              <th>"City"</th>
              <th>"Region"</th>
            </tr>
            <tr>
              <td>{coords}</td>
              <td>{postcode}</td>
              <td>{city}</td>
              <td>{region}</td>
            </tr>
            </table>
        </div>
    }
}
async fn get_endpoint(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // let kv = ctx.kv("KVSTORE")?;

    // let test_val = kv.get("prevkey").text().await?.unwrap();

    let html = leptos::ssr::render_to_string(|cx| {
        view! {cx, <SimpleCounter req />
        }
    });

    return Response::from_html(html);
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router.get_async("/", get_endpoint).run(req, env).await
}
