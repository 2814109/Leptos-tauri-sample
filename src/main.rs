mod components;
use components::{app::*, sample::*};
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
            <Sample/>
        }
    })
}
