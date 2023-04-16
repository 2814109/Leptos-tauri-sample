mod app;
mod sample;

use app::*;
use leptos::*;
use sample::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
            <Sample/>
        }
    })
}
