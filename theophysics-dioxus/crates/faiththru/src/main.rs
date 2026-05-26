use common::{components::ArticleView, types::Article};
use dioxus::prelude::*;

const A1: &str = include_str!("../data/gtq-001.json");
const A2: &str = include_str!("../data/mda-001.json");
const A3: &str = include_str!("../data/duality-001.json");

fn main() {
    dioxus::launch(App);
}

fn load_articles() -> Vec<Article> {
    [A1, A2, A3]
        .iter()
        .map(|raw| serde_json::from_str::<Article>(raw).expect("valid article json"))
        .collect()
}

#[component]
fn App() -> Element {
    let articles = use_memo(load_articles);
    let mut idx = use_signal(|| 0usize);
    let list = articles.read();
    let slugs = list.iter().map(|a| a.id.clone()).collect::<Vec<_>>();
    let current = list[idx()].clone();

    rsx! {
        main { class: "container",
            nav {
                button { onclick: move |_| idx.set(0), "GTQ" }
                button { onclick: move |_| idx.set(1), "MDA" }
                button { onclick: move |_| idx.set(2), "Duality" }
            }
            ArticleView { article: current, all_slugs: slugs }
        }
    }
}
