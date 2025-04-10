use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

fn main() {
    let doc = web_sys::window().unwrap().document().unwrap();

    // Inject stylesheets
    if let Some(head) = doc.head() {
        let css = doc.create_element("link").unwrap();
        css.set_attribute("rel", "stylesheet").unwrap();
        css.set_attribute("href", "assets/tailwind.css").unwrap();
        head.append_child(&css).ok();

        let icon = doc.create_element("link").unwrap();
        icon.set_attribute("rel", "icon").unwrap();
        icon.set_attribute("type", "image/svg+xml").unwrap();
        icon.set_attribute("href", "assets/favicon.svg").unwrap();
        head.append_child(&icon).ok();
    }

    // GoatCounter analytics
    let head = doc.head().unwrap();
    let gc = doc.create_element("script").unwrap();
    gc.set_attribute("data-goatcounter", "https://diegoquinas.goatcounter.com/count").unwrap();
    gc.set_attribute("async", "").unwrap();
    gc.set_attribute("src", "//gc.zgo.at/count.js").unwrap();
    head.append_child(&gc).ok();

    // Setup hamburger toggle (direct DOM, no Dioxus dependency)
    if let Some(h) = doc.get_element_by_id("hamburger") {
        let doc_c = doc.clone();
        let h2 = h.clone();
        let c: Closure<dyn FnMut()> = Closure::new(move || {
            let mm = doc_c.get_element_by_id("mobileMenu").unwrap();
            let mm_el = mm.clone().dyn_into::<web_sys::HtmlElement>().ok();
            let is_open = mm_el.as_ref().and_then(|m| m.style().get_property_value("display").ok()).as_deref() != Some("flex");
            if let Some(m) = &mm_el {
                m.style().set_property("display", if is_open { "flex" } else { "none" }).ok();
            }
            if let Some(body) = doc_c.body() {
                body.style().set_property("overflow", if is_open { "hidden" } else { "" }).ok();
            }
            let h_class = if is_open { "md:hidden flex flex-col justify-center w-6 h-[18px] relative hamburger active cursor-pointer" } else { "md:hidden flex flex-col justify-center w-6 h-[18px] relative hamburger cursor-pointer" };
            h2.set_attribute("class", h_class).ok();
        });
        h.add_event_listener_with_callback("click", c.as_ref().unchecked_ref()).ok();
        c.forget();
    }

    // Close mobile menu on link click
    if let Some(mm) = doc.get_element_by_id("mobileMenu") {
        if let Ok(links) = mm.query_selector_all("a") {
            let mm_el = mm.clone().dyn_into::<web_sys::HtmlElement>().ok();
            for i in 0..links.length() {
                if let Some(link) = links.item(i) {
                    let doc_c = doc.clone();
                    let mm_e = mm_el.clone();
        let c: Closure<dyn FnMut()> = Closure::new(move || {
                        if let Some(m) = &mm_e {
                            m.style().set_property("display", "none").ok();
                        }
                        if let Some(body) = doc_c.body() {
                            body.style().set_property("overflow", "").ok();
                        }
                    });
                    link.add_event_listener_with_callback("click", c.as_ref().unchecked_ref()).ok();
                    c.forget();
                }
            }
        }
    }

    dioxus::launch(diegoquinas_rs::App);
}
