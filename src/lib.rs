use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, IntersectionObserver, IntersectionObserverInit};

#[cfg(target_arch = "wasm32")]
fn use_scroll_effect(scrolled: Signal<bool>) {
    use_effect(move || {
        let w = window().unwrap();
        let mut s = scrolled;
        let w2 = w.clone();
        let c = Closure::new(move || {
            s.set(w2.scroll_y().unwrap_or(0.0) > 50.0);
        });
        w.set_onscroll(Some(c.as_ref().unchecked_ref()));
        c.forget();
    });
}



#[cfg(target_arch = "wasm32")]
fn use_reveal_animations() {
    use_effect(move || {
        let doc = window().unwrap().document().unwrap();
        let selectors = [
            ".bento-card",
            ".timeline-item",
            ".project-card",
            ".contact-card",
            ".section-lead",
        ];
        let mut all: Vec<web_sys::Element> = Vec::new();
        for sel in &selectors {
            if let Ok(nodes) = doc.query_selector_all(sel) {
                for i in 0..nodes.length() {
                    if let Some(node) = nodes.item(i) {
                        if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                            all.push(el);
                        }
                    }
                }
            }
        }
        let mut opts = IntersectionObserverInit::new();
        opts.set_threshold(&wasm_bindgen::JsValue::from_f64(0.1));
        opts.set_root_margin("0px 0px -40px 0px");
        let c: Closure<dyn FnMut(Vec<wasm_bindgen::JsValue>, wasm_bindgen::JsValue)> = Closure::new(
            move |entries: Vec<wasm_bindgen::JsValue>, _: wasm_bindgen::JsValue| {
                for entry in entries.iter() {
                    if let Ok(e) = entry.clone().dyn_into::<web_sys::IntersectionObserverEntry>() {
                        if e.is_intersecting() {
                            if let Ok(el) = e.target().dyn_into::<web_sys::Element>() {
                                let cur = el.get_attribute("class").unwrap_or_default();
                                let cls = format!("{} visible", cur);
                                el.set_attribute("class", &cls).ok();
                            }
                        }
                    }
                }
            },
        );
        let observer = IntersectionObserver::new_with_options(
            c.as_ref().unchecked_ref(),
            &opts,
        )
        .unwrap();
        c.forget();
        for el in &all {
            let cur = el.get_attribute("class").unwrap_or_default();
            let cls = format!("{} reveal", cur);
            el.set_attribute("class", &cls).ok();
            observer.observe(el);
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn use_active_nav(active_section: Signal<String>) {
    use_effect(move || {
        let doc = window().unwrap().document().unwrap();
        let sections = doc.query_selector_all("section[id]").unwrap();
        let mut as_ = active_section;
        let c = Closure::new(move |entries: Vec<wasm_bindgen::JsValue>, _observer: wasm_bindgen::JsValue| {
            for entry in entries.iter() {
                if let Ok(entry) = entry.clone().dyn_into::<web_sys::IntersectionObserverEntry>() {
                    if entry.is_intersecting() {
                        if let Some(id) = entry.target().get_attribute("id") {
                            as_.set(id);
                        }
                    }
                }
            }
        });
        let opts = IntersectionObserverInit::new();
        opts.set_threshold(&wasm_bindgen::JsValue::from_f64(0.3));
        opts.set_root_margin("-80px 0px -50% 0px");
        let observer = IntersectionObserver::new_with_options(c.as_ref().unchecked_ref(), &opts).unwrap();
        c.forget();
        for i in 0..sections.length() {
            if let Some(s) = sections.item(i) {
                if let Some(el) = s.dyn_into::<web_sys::Element>().ok() {
                    observer.observe(&el);
                }
            }
        }
    });
}

pub fn App() -> Element {
    #[cfg(target_arch = "wasm32")]
    use_reveal_animations();

    rsx! {
        NavBar {}
        HeroSection {}
        AboutSection {}
        ExperienceSection {}
        OpenSourceSection {}
        ProjectsSection {}
        ContactSection {}
        Footer {}
    }
}

#[component]
fn NavBar() -> Element {
    let scrolled = use_signal(|| false);
    let active_section = use_signal(|| "hero".to_string());

    #[cfg(target_arch = "wasm32")]
    use_scroll_effect(scrolled);

    #[cfg(target_arch = "wasm32")]
    use_active_nav(active_section);

    let bar = if scrolled() { "border-zinc-800" } else { "border-transparent" };

    rsx! {
        nav { id: "nav", class: "fixed inset-x-0 top-0 z-50 bg-black/70 backdrop-blur-xl border-b {bar} transition-colors",
            div { class: "max-w-5xl mx-auto px-6 h-[72px] flex items-center justify-between",
                a { href: "#hero", class: "text-xl font-extrabold tracking-tight",
                    "dpg"
                    span { class: "text-orange-500", "." }
                }
                div { class: "hidden md:flex gap-8",
                    NavLink { href: "#about", label: "About", active: active_section() == "about" }
                    NavLink { href: "#experience", label: "Experience", active: active_section() == "experience" }
                    NavLink { href: "#opensource", label: "Open Source", active: active_section() == "opensource" }
                    NavLink { href: "#projects", label: "Projects", active: active_section() == "projects" }
                    NavLink { href: "#contact", label: "Contact", active: active_section() == "contact" }
                    a { href: "/blog/", class: "text-sm font-medium text-zinc-400 hover:text-white transition-colors", "Blog" }
                }
                button { id: "hamburger", class: "md:hidden flex flex-col justify-center w-6 h-[18px] relative hamburger cursor-pointer",
                    span { class: "hamburger-line" }
                    span { class: "hamburger-line" }
                    span { class: "hamburger-line" }
                }
            }
        }
        div { id: "mobileMenu", class: "fixed inset-x-0 top-[72px] bottom-0 bg-black z-40 hidden flex-col items-center justify-center gap-8",
            a { href: "#about", class: "text-2xl font-semibold text-zinc-400 hover:text-white transition-colors", "About" }
            a { href: "#experience", class: "text-2xl font-semibold text-zinc-400 hover:text-white transition-colors", "Experience" }
            a { href: "#opensource", class: "text-2xl font-semibold text-zinc-400 hover:text-white transition-colors", "Open Source" }
            a { href: "#projects", class: "text-2xl font-semibold text-zinc-400 hover:text-white transition-colors", "Projects" }
            a { href: "#contact", class: "text-2xl font-semibold text-zinc-400 hover:text-white transition-colors", "Contact" }
            a { href: "/blog/", class: "text-2xl font-semibold text-zinc-400 hover:text-white transition-colors", "Blog" }
        }
    }
}

#[component]
fn NavLink(href: String, label: String, active: bool) -> Element {
    let cls = if active {
        "text-sm font-medium text-white transition-colors"
    } else {
        "text-sm font-medium text-zinc-400 hover:text-white transition-colors"
    };
    rsx! { a { href: "{href}", class: "{cls}", "{label}" } }
}

#[component]
fn HeroSection() -> Element {
    rsx! {
        section { id: "hero", class: "min-h-[90vh] flex flex-col justify-center px-6 pt-[120px] pb-[60px] max-w-5xl mx-auto",
            div { class: "font-mono text-xs text-orange-500 mb-6 tracking-widest uppercase font-semibold", "Software Engineer" }
            h1 { class: "text-5xl md:text-8xl font-extrabold tracking-tighter leading-none mb-6 max-w-3xl",
                "Diego Perez "
                span { class: "text-zinc-500", "Giordán" }
            }
            p { class: "text-lg md:text-2xl text-zinc-400 max-w-xl mb-12 leading-relaxed",
                "Building distributed systems and high-performance microservices. "
                br {}
                "Currently at Cencosud. Open source contributor to Apache DataFusion."
            }
            div { class: "flex gap-4 items-center",
                a { href: "#about", class: "px-7 py-3.5 rounded-full text-sm font-semibold bg-white text-black hover:scale-105 transition-transform inline-flex items-center justify-center",
                    "Discover my work"
                }
                div { class: "flex gap-4 ml-4",
                    a { href: "https://github.com/diegoQuinas", target: "_blank", rel: "noopener", class: "w-11 h-11 rounded-full border border-zinc-800 flex items-center justify-center text-zinc-400 hover:border-white hover:text-white hover:bg-zinc-900/60 transition-all", "aria-label": "GitHub",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z" }
                        }
                    }
                    a { href: "https://linkedin.com/in/diego-perez-giordan", target: "_blank", rel: "noopener", class: "w-11 h-11 rounded-full border border-zinc-800 flex items-center justify-center text-zinc-400 hover:border-white hover:text-white hover:bg-zinc-900/60 transition-all", "aria-label": "LinkedIn",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AboutSection() -> Element {
    rsx! {
        section { id: "about", class: "py-[120px] px-6",
            div { class: "max-w-5xl mx-auto",
                div { class: "grid grid-cols-4 auto-rows-[minmax(200px,auto)] gap-6",
                    div { class: "bento-card col-span-4 md:col-span-3 row-span-2 justify-between bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 flex flex-col transition-colors hover:border-zinc-700 overflow-hidden relative",
                        h2 { class: "text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-4 font-mono", "Background" }
                        div { class: "text-lg text-zinc-400 leading-relaxed",
                            p { "I'm a Backend Engineer with 4+ years of experience, currently working at "
                                strong { class: "text-white font-medium", "Cencosud" }
                                " (top-3 LATAM retailer). I build "
                                strong { class: "text-white font-medium", "Go" }
                                " microservices for a multi-banner e-commerce platform that serves 5+ countries, optimizing highly concurrent APIs handling massive retail traffic."
                            }
                            p { class: "mt-4", "Beyond my daily work, I contribute to "
                                strong { class: "text-white font-medium", "Apache DataFusion" }
                                " in "
                                strong { class: "text-white font-medium", "Rust" }
                                ", diving deep into columnar processing and high-performance execution plans. I strongly advocate for compile-time safety, clean architecture, and TDD."
                            }
                        }
                    }
                    div { class: "bento-card col-span-2 md:col-span-1 row-span-1 flex items-center justify-center bg-zinc-900/60 rounded-2xl overflow-hidden aspect-square",
                        img {
    src: {
        #[cfg(target_arch = "wasm32")]
        { asset!("/assets/diego-profile-rust.jpeg") }
        #[cfg(not(target_arch = "wasm32"))]
        { "assets/diego-profile-rust.jpeg" }
    },
    alt: "Diego Perez Giordán",
    loading: "lazy",
    class: "w-full h-full object-cover rounded-full",
}
                    }
                    div { class: "bento-card col-span-2 md:col-span-1 row-span-1 flex flex-col items-center justify-center text-center bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 transition-colors hover:border-zinc-700",
                        div { class: "w-12 h-12 rounded-full bg-zinc-800 flex items-center justify-center mb-4 text-white",
                            svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", "stroke-width": "2", "stroke-linecap": "round", "stroke-linejoin": "round",
                                path { d: "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" }
                                circle { cx: "12", cy: "10", r: "3" }
                            }
                        }
                        h3 { class: "text-lg mb-1", "Uruguay" }
                        p { class: "text-sm text-zinc-400", "GMT overlap con US" }
                    }
                    div { class: "bento-card col-span-4 row-span-1 bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 flex flex-col transition-colors hover:border-zinc-700",
                        h2 { class: "text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-4 font-mono", "Core Technologies" }
                        div { class: "grid grid-cols-[repeat(auto-fit,minmax(150px,1fr))] gap-6 mt-2",
                            StackItem { title: "Languages".to_string(), items: vec!["Go (Production)".to_string(), "Rust (OSS & Personal)".to_string(), "Node.js / TS".to_string()], highlight: vec![true, true, false] }
                            StackItem { title: "Backend".to_string(), items: vec!["gRPC & Protobuf".to_string(), "PostgreSQL & MySQL".to_string(), "Clean Architecture".to_string()], highlight: vec![false, false, false] }
                            StackItem { title: "Infrastructure".to_string(), items: vec!["Docker & Kubernetes".to_string(), "GCP".to_string(), "Tilt & CI/CD".to_string()], highlight: vec![false, false, false] }
                            StackItem { title: "Environment".to_string(), items: vec!["Linux (Arch)".to_string(), "Neovim / Zellij".to_string(), "Claude Code / Agents".to_string()], highlight: vec![false, false, false] }
                        }
                    }
                }
            }
        }
    }
}

#[component]
#[component]
fn StackItem(title: String, items: Vec<String>, highlight: Vec<bool>) -> Element {
    rsx! {
        div {
            h4 { class: "text-base font-semibold text-white mb-3", "{title}" }
            ul { class: "flex flex-col gap-2",
                for (i, item) in items.iter().enumerate() {
                    li { class: "flex items-center gap-2 text-sm", style: if highlight.get(i).copied().unwrap_or(false) { "color: #f5f5f5" } else { "color: #a3a3a3" },
                        span { class: "w-1 h-1 rounded-full bg-zinc-500 shrink-0" }
                        "{item}"
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceSection() -> Element {
    rsx! {
        section { id: "experience", class: "py-[120px] px-6",
            div { class: "max-w-5xl mx-auto",
                h2 { class: "text-4xl md:text-6xl font-extrabold tracking-tight mb-16", "Experience." }
                div { class: "flex flex-col gap-6",
                    TimelineItem {
                        title: "Backend Engineer".to_string(),
                        company: "Cencosud S.A. - Digital Commerce".to_string(),
                        date: "Dec 2025 - Present".to_string(),
                        bullets: vec![
                            "Develop Go microservices and REST/gRPC APIs powering the WhiteLabel platform serving 5+ countries (Jumbo, Disco, Paris).".to_string(),
                            "Migrated 15 legacy endpoints to a decoupled architecture, reducing response latency by up to 60% through concurrent execution with goroutines and semaphore-based rate limiting.".to_string(),
                            "Apply TDD and clean architecture principles with PostgreSQL as the primary data store.".to_string(),
                        ],
                        tags: vec!["Go".to_string(), "gRPC".to_string(), "Kubernetes".to_string(), "GCP".to_string(), "PostgreSQL".to_string()],
                    }
                    TimelineItem {
                        title: "QA Automation Engineer".to_string(),
                        company: "Cencosud S.A. - Digital Commerce".to_string(),
                        date: "Apr 2023 - Dec 2025".to_string(),
                        bullets: vec![
                            "Designed and maintained automated test suites for multi-banner mobile applications across LATAM.".to_string(),
                            "Built scripts and automated test flows that improved coverage and reduced regression cycle time.".to_string(),
                            "Developed deep understanding of the platform\'s architecture and APIs, enabling a smooth transition into backend engineering.".to_string(),
                        ],
                        tags: vec!["QA".to_string(), "Test Automation".to_string(), "API Testing".to_string()],
                    }
                    TimelineItem {
                        title: "Backend Developer Trainee".to_string(),
                        company: "Cencosud S.A. - Digital Commerce".to_string(),
                        date: "Aug 2022 - Apr 2023".to_string(),
                        bullets: vec![
                            "Built REST APIs in Node.js (Express, Nest) with MySQL for the Paris Chile e-commerce platform.".to_string(),
                        ],
                        tags: vec!["Node.js".to_string(), "Express".to_string(), "MySQL".to_string()],
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineItem(title: String, company: String, date: String, bullets: Vec<String>, tags: Vec<String>) -> Element {
    rsx! {
        div { class: "timeline-item bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 transition-all hover:border-zinc-700 hover:bg-zinc-900",
            div { class: "flex flex-col md:flex-row justify-between md:items-start gap-3 mb-5",
                div {
                    h3 { class: "text-xl font-bold mb-1", "{title}" }
                    div { class: "text-[15px] text-orange-500 font-medium", "{company}" }
                }
                div { class: "font-mono text-sm text-zinc-500 px-3 py-1 bg-black rounded-full border border-zinc-800 shrink-0", "{date}" }
            }
            div {
                ul { class: "flex flex-col gap-3",
                    for bullet in &bullets {
                        li { class: "flex items-start gap-2 text-zinc-400 text-sm",
                            span { class: "font-mono text-zinc-500 shrink-0", "->" }
                            "{bullet}"
                        }
                    }
                }
                div { class: "flex flex-wrap gap-2 mt-6",
                    for tag in &tags {
                        span { class: "font-mono text-xs px-3 py-1 bg-black border border-zinc-800 rounded-md text-zinc-400", "{tag}" }
                    }
                }
            }
        }
    }
}

#[component]
fn OpenSourceSection() -> Element {
    rsx! {
        section { id: "opensource", class: "py-[120px] px-6",
            div { class: "max-w-5xl mx-auto",
                h2 { class: "text-4xl md:text-6xl font-extrabold tracking-tight mb-16", "Open Source." }
                p { class: "section-lead text-lg md:text-xl text-zinc-400 max-w-2xl -mt-10 mb-16 leading-relaxed",
                    "Active contributor to high-performance and privacy-focused tools in the Rust ecosystem. Focus on "
                    span { class: "text-white", "Apache DataFusion" }
                    " and "
                    span { class: "text-white", "ApiArk" }
                    "."
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    a { href: "https://github.com/berbicanes/apiark/pulls?q=author%3AdiegoQuinas", target: "_blank", rel: "noopener", class: "project-card col-span-full bg-gradient-to-br from-zinc-900/60 to-[#161210] border border-zinc-700 rounded-2xl p-8 flex flex-col transition-all hover:border-zinc-600 hover:bg-zinc-900/80",
                        div { class: "flex justify-between items-start mb-4",
                            div { class: "font-mono text-xs text-zinc-500", "berbicanes/apiark" }
                            div { class: "text-xs font-semibold px-3 py-1 rounded-full bg-orange-500/15 text-orange-500", "Active Core Contributor" }
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-[1fr_250px] gap-10",
                            div {
                                h3 { class: "text-xl md:text-2xl font-bold mb-4 leading-tight", "Tauri Privacy-First API Client" }
                                p { class: "text-sm text-zinc-400 leading-relaxed", "10+ merged and open PRs to ApiArk - a lightweight Postman alternative with no cloud sync and low memory footprint (~60 MB). Implemented major features including the \"open requests\" tab system, gRPC server reflection discovery, custom borderless title bars, environment variable interpolation, and JSONC parsing support." }
                                div { class: "flex flex-wrap gap-2 mt-6",
                                    ProjectTag { label: "Tauri" }
                                    ProjectTag { label: "Rust" }
                                    ProjectTag { label: "TypeScript" }
                                    ProjectTag { label: "gRPC" }
                                }
                            }
                            div { class: "bg-black border border-zinc-800 rounded-lg p-5 font-mono text-xs",
                                div { class: "flex justify-between py-2 border-b border-dashed border-zinc-800",
                                    span { class: "text-zinc-500", "Role" }
                                    span { class: "text-white text-right", "Contributor" }
                                }
                                div { class: "flex justify-between py-2 border-b border-dashed border-zinc-800",
                                    span { class: "text-zinc-500", "Contributions" }
                                    span { class: "text-white text-right", "10+ PRs" }
                                }
                                div { class: "flex justify-between py-2",
                                    span { class: "text-zinc-500", "Impact" }
                                    span { class: "text-white text-right", "Core Features" }
                                }
                            }
                        }
                    }
                    OSCard {
                        href: "https://github.com/apache/datafusion/pull/21710",
                        repo: "apache/datafusion",
                        badge: "Approved",
                        badge_color: "text-orange-500 bg-orange-500/15",
                        title: rsx! { span { "2-arg ", span { class: "code", "ceil()" }, " for Spark Compat" } },
                        desc: rsx! { span { "Extended ", span { class: "code", "datafusion-spark" }, "'s ceiling function to accept a scale parameter (", span { class: "code", "ceil(3.145, 2) -> 3.15" }, "), matching Apache Spark SQL semantics exactly. Rewrote function signatures and dynamic return types." } },
                        tags: vec!["Rust".to_string(), "SQL".to_string(), "Query Engine".to_string()],
                    }
                    OSCard {
                        href: "https://github.com/apache/datafusion/pull/22643",
                        repo: "apache/datafusion",
                        badge: "Merged",
                        badge_color: "text-emerald-400 bg-emerald-400/10",
                        title: rsx! { span { "Fix flaky ", span { class: "code", "sqllogictest" }, " filter" } },
                        desc: rsx! { span { "Made parallel aggregate plan tests deterministic by removing content-dependent snapshot assertions from ", span { class: "code", "DynamicFilter" }, " EXPLAIN outputs. Fixed same-day as reported." } },
                        tags: vec!["Rust".to_string(), "Testing".to_string()],
                    }
                }
            }
        }
    }
}

#[component]
fn OSCard(href: String, repo: String, badge: String, badge_color: String, title: Element, desc: Element, tags: Vec<String>) -> Element {
    rsx! {
        a { href: "{href}", target: "_blank", rel: "noopener", class: "bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 flex flex-col transition-all hover:border-zinc-700 hover:bg-zinc-900",
            div { class: "flex justify-between items-start mb-4",
                div { class: "font-mono text-xs text-zinc-500", "{repo}" }
                div { class: "text-xs font-semibold px-3 py-1 rounded-full {badge_color}", "{badge}" }
            }
            h3 { class: "text-xl md:text-2xl font-bold mb-4 leading-tight", {title} }
            p { class: "text-sm text-zinc-400 leading-relaxed flex-1", {desc} }
            div { class: "flex flex-wrap gap-2 mt-6",
                for tag in &tags {
                    span { class: "font-mono text-xs px-3 py-1 bg-black border border-zinc-800 rounded-md text-zinc-400", "{tag}" }
                }
            }
        }
    }
}

#[component]
fn ProjectTag(label: String) -> Element {
    rsx! { span { class: "font-mono text-xs px-3 py-1 bg-black border border-zinc-800 rounded-md text-zinc-400", "{label}" } }
}

#[component]
fn ProjectsSection() -> Element {
    rsx! {
        section { id: "projects", class: "py-[120px] px-6",
            div { class: "max-w-5xl mx-auto",
                h2 { class: "text-4xl md:text-6xl font-extrabold tracking-tight mb-16", "Projects." }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    ProjectCard {
                        repo: "diegoQuinas/oxidia".to_string(),
                        badge: Some("Active".to_string()),
                        title: "Oxidia Game Server".to_string(),
                        description: "An Open Tibia server written in Rust (protocol 10.98, OTClient Redemption). Uses Tokio async runtime for networking, custom protocol packet handling, and highly concurrent in-memory world state management.".to_string(),
                        tags: vec!["Rust".to_string(), "Tokio".to_string(), "MMORPG Architecture".to_string()],
                        href: Some("https://github.com/diegoQuinas/oxidia".to_string()),
                    }
                    ProjectCard {
                        repo: "Personal Learning".to_string(),
                        badge: Some("In Progress".to_string()),
                        title: "Minifusion".to_string(),
                        description: "Building a DataFusion-inspired analytical query engine from scratch in Rust. Focused on deepening my understanding of columnar memory processing, vectorized expression evaluation, and physical execution plans.".to_string(),
                        tags: vec!["Rust".to_string(), "Columnar Databases".to_string()],
                        href: None::<String>,
                    }
                    ProjectCard {
                        repo: "diegoQuinas/liturgia-horas-tui".to_string(),
                        badge: None::<String>,
                        title: "Liturgia Horas TUI".to_string(),
                        description: "Terminal UI application built in Rust for praying the Liturgy of the Hours. Offline-first design with a clean, fully keyboard-driven interface using Ratatui.".to_string(),
                        tags: vec!["Rust".to_string(), "Ratatui".to_string(), "Terminal UI".to_string()],
                        href: Some("https://github.com/diegoQuinas/liturgia-horas-tui".to_string()),
                    }
                    ProjectCard {
                        repo: "diegoQuinas/test_case_manager".to_string(),
                        badge: None::<String>,
                        title: "Test Case Manager".to_string(),
                        description: "CLI-based test case manager that generates clean Markdown and CSV reports. Built in Rust to bridge my QA automation background with systems programming.".to_string(),
                        tags: vec!["Rust".to_string(), "CLI".to_string(), "Tooling".to_string()],
                        href: Some("https://github.com/diegoQuinas/test_case_manager".to_string()),
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(repo: String, badge: Option<String>, title: String, description: String, tags: Vec<String>, href: Option<String>) -> Element {
    let inner = rsx! {
        div { class: "flex justify-between items-start mb-4",
            div { class: "font-mono text-xs text-zinc-500", "{repo}" }
            if let Some(b) = &badge {
                div { class: "text-xs font-semibold px-3 py-1 rounded-full bg-orange-500/15 text-orange-500", "{b}" }
            }
        }
        h3 { class: "text-xl md:text-2xl font-bold mb-4 leading-tight", "{title}" }
        p { class: "text-sm text-zinc-400 leading-relaxed flex-1", "{description}" }
        div { class: "flex flex-wrap gap-2 mt-6",
            for tag in &tags {
                ProjectTag { label: tag }
            }
        }
    };

    if let Some(href) = &href {
        rsx! {
        a { href: "{href}", target: "_blank", rel: "noopener", class: "project-card bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 flex flex-col transition-all hover:border-zinc-700 hover:bg-zinc-900",
                {inner}
            }
        }
    } else {
        rsx! {
            div { class: "project-card bg-zinc-900/60 border border-zinc-800 rounded-2xl p-8 flex flex-col transition-all hover:border-zinc-700 hover:bg-zinc-900",
                {inner}
            }
        }
    }
}

#[component]
fn ContactSection() -> Element {
    rsx! {
        section { id: "contact", class: "py-[120px] px-6",
            div { class: "max-w-5xl mx-auto",
                h2 { class: "text-4xl md:text-6xl font-extrabold tracking-tight mb-16", "Let's Connect." }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    a { href: "mailto:diegoperezgiordan@gmail.com", class: "contact-card group bg-zinc-900/60 border border-zinc-800 rounded-2xl p-10 flex flex-col items-center justify-center text-center gap-4 transition-all hover:border-white hover:bg-white hover:text-black",
                        svg { class: "w-8 h-8 group-hover:text-black", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", "stroke-width": "2",
                            rect { x: "2", y: "4", width: "20", height: "16", rx: "2" }
                            path { d: "M22 7l-10 7L2 7" }
                        }
                        span { class: "text-lg font-semibold text-white group-hover:text-black", "diegoperezgiordan@gmail.com" }
                    }
                    a { href: "https://linkedin.com/in/diego-perez-giordan", target: "_blank", rel: "noopener", class: "contact-card group bg-zinc-900/60 border border-zinc-800 rounded-2xl p-10 flex flex-col items-center justify-center text-center gap-4 transition-all hover:border-white hover:bg-white hover:text-black",
                        svg { class: "w-8 h-8 group-hover:text-black", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                        }
                        span { class: "text-lg font-semibold text-white group-hover:text-black", "LinkedIn Profile" }
                    }
                    a { href: "https://github.com/diegoQuinas", target: "_blank", rel: "noopener", class: "contact-card group bg-zinc-900/60 border border-zinc-800 rounded-2xl p-10 flex flex-col items-center justify-center text-center gap-4 transition-all hover:border-white hover:bg-white hover:text-black",
                        svg { class: "w-8 h-8 group-hover:text-black", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z" }
                        }
                        span { class: "text-lg font-semibold text-white group-hover:text-black", "GitHub @diegoQuinas" }
                    }
                    a { href: "https://bsky.app/profile/diegoquinas.bsky.social", target: "_blank", rel: "noopener", class: "contact-card group bg-zinc-900/60 border border-zinc-800 rounded-2xl p-10 flex flex-col items-center justify-center text-center gap-4 transition-all hover:border-white hover:bg-white hover:text-black",
                        svg { class: "w-8 h-8 group-hover:text-black", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M5.07 3.677c2.696 2.021 5.596 6.119 6.93 8.318 1.334-2.199 4.234-6.297 6.93-8.318 1.944-1.458 5.07-2.586 5.07.981 0 .712-.408 5.984-.648 6.838-.834 2.969-3.86 3.726-6.546 3.27 4.696.799 5.892 3.447 3.313 6.094-4.9 5.026-7.043-1.262-7.591-2.871-.101-.295-.148-.434-.148-.315 0-.119-.047.02-.148.315-.547 1.609-2.69 7.897-7.59 2.871-2.58-2.647-1.384-5.295 3.312-6.094-2.686.456-5.712-.301-6.546-3.27C.566 10.642.158 5.37.158 4.658c0-3.567 3.126-2.439 5.07-.981z" }
                        }
                        span { class: "text-lg font-semibold text-white group-hover:text-black", "Bluesky" }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "py-10 px-6 text-center border-t border-zinc-800",
            p { class: "font-mono text-xs text-zinc-500", "Designed and built by Diego Perez Giordán." }
            div { class: "mt-4", style: "display: flex; justify-content: center;",
                span { class: "font-mono text-[10px] px-2 py-1 rounded-full bg-orange-500/10 text-orange-500 border border-orange-500/20", "Made with Dioxus SSR" }
            }
        }
    }
}
