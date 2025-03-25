#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        #[cfg(feature = "web")]
        // This should never run, but it does if you serve with `dx serve`
        // It doesn't if you run with `dx serve --platform web`
        panic!();
    }
    launch(|| rsx!{"hi"});
}
