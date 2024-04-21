#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));
const LINK_BUTTON_CLASSES: &str = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800 h-full";

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "bg-gradient-to-r from-cyan-500 to-blue-500 height-vdh",
            img { class: "mx-auto",
                src: "shrune.svg",
                width: 300,
                height: 300,
            }
            div { class: "text-center",
                div { class: "text-5xl p-5",
                    "Welcome to Shrunes!"
                }
                div { class: "text-2xl",
                    "Shrunes are a mushroom inspired memecoin built on the runes metaprotocol of bitcoin."
                }
                div { class: "p-5",
                    a { 
                        class: LINK_BUTTON_CLASSES,
                        href: "https://luminex.io/rune/WWW%E2%80%A2SHRUNES%E2%80%A2COM",
                        target: "_blank",
                        "Mint Shrunes on Luminex"
                    }
                }
                div { class: "p-5",
                    a {
                        class: LINK_BUTTON_CLASSES,
                        href: "https://mempool.space/",
                        target: "_blank",
                        "View the mempool"
                    }
                }
                div { class: "p-5",
                    a {
                        class: LINK_BUTTON_CLASSES,
                        href: "https://unisat.io/runes/market?tick=WWW%E2%80%A2SHRUNES%E2%80%A2COM",
                        target: "_blank",
                        "Buy/Sell on Unisat (secondary market)" 
                    }
                }
                div { class: "p-5",
                    a {
                        class: LINK_BUTTON_CLASSES,
                        href: "https://twitter.com/gamedevalice",
                        target: "_blank",
                        "Follow on Twitter" 
                    }
                }
                div { class: "p-5",
                    a {
                        class: LINK_BUTTON_CLASSES,
                        href: "https://discord.gg/xNsS3s2e9p",
                        target: "_blank",
                        "Join the Discord Server" 
                    }
                }
                div { class: "p-5",
                    a {
                        class: LINK_BUTTON_CLASSES,
                        href: "https://github.com/shrunes",
                        target: "_blank",
                        "Github" 
                    }
                }
                div {
                    class: "h-40"
                }
            }

        }
        
    // Link {
    //     to: Route::Blog {
    //         id: count()
    //     },
    //     "Go to blog"
    // }
    }
}
