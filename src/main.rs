use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!(
        document::Link { rel: "icon", href: FAVICON },
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        Home {}
    )
}

#[component]
pub fn Home() -> Element {
    rsx! {
        Inputs {}
    }
}

#[component]
pub fn Inputs() -> Element {
    let mut component_view = use_signal(|| "hourly".to_string());

    let mut handle_paid_click = move |event: Event<MouseData>, label: &str| {
        component_view.set(label.to_string());
        println!("{}", label);
    };

    rsx! {
       h2 { "How are you paid?" }
       div { class: "button-wrapper",
            button { id: "hourly-button", onclick: move |event| handle_paid_click(event, "hourly"), "Hourly" }
            button { id: "yearly-button", onclick: move |event| handle_paid_click(event, "yearly"), "Yearly" }
       }
       h3 { "{component_view}" }
       
       if component_view() == "hourly" {
            h3 {
                "Hourly View"
            }
       } else {
            h3 {
                "Yearly View"
            }
       }
    }
}
