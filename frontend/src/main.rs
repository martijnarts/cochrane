use dioxus::prelude::fc_to_builder;
use dioxus::{
    core::IntoDynNode,
    core_macro::Props,
    hooks::use_future,
    prelude::{dioxus_elements, rsx, use_state, Element, Scope},
};
use dioxus_fullstack::prelude::LaunchBuilder;
use serde::Deserialize;

#[cfg(feature = "ssr")]
fn main() {
    tracing_subscriber::fmt::init();

    LaunchBuilder::new(app)
        .addr(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::new(0, 0, 0, 0),
            8080,
        ))
        .launch();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    LaunchBuilder::new(app).launch();
}

#[derive(Deserialize)]
struct DioxusEnv {
    backend_url: url::Url,
}

#[derive(PartialEq, Props)]
struct QueryingTextProps {
    backend_url: url::Url,
}

#[allow(non_snake_case)]
fn QueryingText(cx: Scope<QueryingTextProps>) -> Element {
    let result = use_future(cx, &cx.props.backend_url, |val| async move {
        let res = call_api(val).await;
        res
    });

    cx.render(match result.value() {
        Some(Ok(s)) => rsx! { p { "{s}" } },
        Some(Err(e)) => rsx! { p { "{e}" } },
        None => rsx! { p { "Loading..." } },
    })
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let value = get_dioxus_env(&cx);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        match value {
            Some(v) => match v {
                Ok(DioxusEnv { backend_url }) => rsx! { QueryingText { backend_url: backend_url } },
                Err(e) => rsx! { p { "{e}" } }
            },
            None => rsx! { p { "Loading..." } }
        }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}

async fn call_api(url: url::Url) -> Result<String, String> {
    let res = reqwest::get(url).await.map_err(|e| e.to_string())?;
    res.text().await.map_err(|e| e.to_string())
}

// For SSR, we don't use the cx
#[allow(unused_variables)]
fn get_dioxus_env(cx: &Scope) -> Option<Result<DioxusEnv, String>> {
    let json_str = {
        #[cfg(feature = "ssr")]
        {
            Some(std::env::var("DIOXUS_ENV").map_err(|e| e.to_string()))
        }
        #[cfg(not(feature = "ssr"))]
        {
            use dioxus::events::use_eval;
            use dioxus::hooks::to_owned;

            let create_eval = use_eval(cx);
            let eval = create_eval(r#"dioxus.send(env);"#).unwrap();
            let future = use_future(cx, (), |_| {
                to_owned![eval];
                async move {
                    // You can receive any message from JavaScript with the recv method
                    eval.recv().await.unwrap()
                }
            });

            future.value().map(|val| val.to_string()).map(Result::Ok)
        }
    };

    json_str.map(|res| res.and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string())))
}
