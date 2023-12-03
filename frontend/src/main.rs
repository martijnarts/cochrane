use dioxus::prelude::{dioxus_elements, rsx, use_state, Element, Scope};
use dioxus_fullstack::prelude::LaunchBuilder;

#[cfg(feature = "ssr")]
fn main() {
    use tracing_subscriber;
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

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}
