use dioxus::prelude::*;

use crate::components::{footer::Footer, nav::Navbar};

pub mod _404;
pub mod blog;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "h-screen bg-cover bg-white dark:bg-gray-600",
            Navbar {}
            div {
                class: "flex h-4/6 w-full items-center justify-center container mx-auto px-8",
                div {
                    class: "max-w-2xl text-center",
                    img {
                        class: "rounded-lg w-32 mb-4 mx-auto",
                        src: "https://avatars.githubusercontent.com/u/74538854?v=4"
                    }
                    h5 {
                        class: "text-2xl dark:text-white font-medium leading-tight mb-2",
                        "Kyros"
                    }
                    p {
                        class: "text-gray-500 dark:text-gray-300",
                        "Computer Science Student | Keen on new technologies."
                    }
                    Footer {}
                }
            }
        }
    })
}

#[derive(Debug, Clone)]
struct Category {
    pub name: &'static str,
    pub projects: Vec<Project>,
}
#[derive(Debug, Clone)]
struct Project {
    pub name: &'static str,
    pub desc: &'static str,
    pub url: &'static str,
    pub job: &'static str,
}

pub fn Projects(cx: Scope) -> Element {
    let data: Vec<Category> = vec![
        Category {
            name: "Web Development",
            projects: vec![
                Project {
                    name: "bt-rust",
                    desc: "BitTorrent download tool in development stage", 
                    url: "https://github.com/kyrosle/bt-rust.git",
                    job: "Author",
                },
            ],
        },
    ];

    let displayer = data.iter().map(|v| {
        rsx! {
            h2 {
                class: "text-xl font-bold",
                "# {v.name}"
            }
            div {
                class: "mt-4 grid md:grid-cols-2 gap-2 mb-8",
                v.projects.iter().map(|p| {
                    rsx! {
                        a {
                            class: "block p-4 rounded-lg shadow-lg bg-white w-64 dark:bg-gray-700 hover:bg-gray-200",
                            href: "{p.url}",
                            target: "_blank",
                            h5 {
                                class: "text-gray-900 dark:text-white text-xl leading-tight font-semibold mb-2",
                                "{p.name}"
                            }
                            p {
                                class: "text-gray-700 dark:text-gray-200 text-base mb-2",
                                "{p.desc}"
                            }
                            p {
                                class: "text-gray-400 dark:text-gray-500 text-base",
                                "{p.job}"
                            }
                        }
                    }
                })
            }
        }
    });

    cx.render(rsx! {
        section {
            class: "bg-cover bg-white dark:bg-gray-600 dark:text-white",
            Navbar {}
            div {
                class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                div {
                    class: "max-w-5xl text-center",
                    displayer
                    Footer {}
                }
            }
        }
    })
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "bg-cover bg-white dark:bg-gray-600 dark:text-white",
            Navbar {}
            div {
                class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                div {
                    class: "max-w-5xl text-center",
                    img {
                        class: "rounded-lg w-32 mb-4 mx-auto",
                        src: "https://avatars.githubusercontent.com/u/74538854?v=4"
                    }
                    div {
                        class: "space-y-4 text-gray-900 dark:text-gray-300",
                        p {
                            "Hi, My name is Kyros, currently I'm a student."
                        }
                        p {
                            "My email - ",
                            a {
                                class: "underline font-bold",
                                "le@90e.com"
                            }
                        }
                        p {
                            "My favorite tech stack - ",
                            strong {
                                "Rust, Go, .net core",
                            }
                        }
                    }
                    Footer {}
                }
            }
        }
    })
}
