use dioxus::prelude::*;
use crate::{theme::accent, types::Article};

#[component]
pub fn Sidebar(current: String, slugs: Vec<String>) -> Element {
    rsx! {
        aside { class: "sidebar",
            h3 { "Series Navigation" }
            ul {
                for slug in slugs {
                    li { class: if slug == current {"active"} else {""}, "{slug}" }
                }
            }
        }
    }
}

#[component]
pub fn TabSystem(article: Article) -> Element {
    let mut active = use_signal(|| "narrative".to_string());
    let current = active();
    let content = match current.as_str() {
        "system" => article.system.clone(),
        "academic" => article.academic.clone(),
        _ => article.narrative.clone(),
    };
    rsx! {
        div {
            button { onclick: move |_| active.set("narrative".into()), "Narrative" }
            button { onclick: move |_| active.set("system".into()), "System" }
            button { onclick: move |_| active.set("academic".into()), "Academic" }
            article { "{content}" }
        }
    }
}

#[component]
pub fn AudioPlayer(audio_url: Option<String>) -> Element {
    rsx! {
        footer { class: "audio-player",
            if let Some(url) = audio_url {
                audio { controls: true, src: "{url}" }
            } else {
                p { "No audio for this article." }
            }
            p { "Speed controls and persistent state are next iteration." }
        }
    }
}

#[component]
pub fn ArticleView(article: Article, all_slugs: Vec<String>) -> Element {
    let color = accent(article.series);
    rsx! {
        div { style: "border-top: 3px solid {color}; padding: 1rem;",
            p { "Reading level: {article.reading_level} · Claims: {article.claims_count}" }
            h1 { "{article.title}" }
            Sidebar { current: article.id.clone(), slugs: all_slugs }
            TabSystem { article: article.clone() }
            AudioPlayer { audio_url: article.audio_url.clone() }
        }
    }
}
