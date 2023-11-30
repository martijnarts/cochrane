use dioxus::prelude::{dioxus_elements, rsx, use_state, Element, Scope};
use dioxus_fullstack::prelude::LaunchBuilder;
use tracing_subscriber;

#[cfg(ssr)]
fn main() {
    tracing_subscriber::fmt::init();

    LaunchBuilder::new(app)
        .addr([[0, 0, 0, 0], 8080].into())
        .launch();
}

#[cfg(not(ssr))]
fn main() {
    tracing_subscriber::fmt::init();

    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}
