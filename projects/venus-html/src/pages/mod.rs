use dioxus::prelude::*;



pub fn ModulePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%;",
            Sidebar(cx),
            MainBody(cx),
        }
    })
}

pub fn MainBody(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: row;",
            "sidebar"
        }
    })
}

// create a component that renders a div with the text "Hello, world!"
pub fn Sidebar(cx: Scope) -> Element {
    const PLACE_HOLDER: &str = "C6H5COOH + O2 = CO2 + H2O";
    let text = use_state(&cx, || PLACE_HOLDER.to_string());
    cx.render(rsx! {
        nav {
            h2 { "This is side bar" }
        }
    })
}
