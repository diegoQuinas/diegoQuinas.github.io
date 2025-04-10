use std::fs;

use dioxus::prelude::*;

fn main() {
    fs::create_dir_all("dist/assets").unwrap();

    let html = render_page();
    fs::write("dist/index.html", html).unwrap();
    println!("  dist/index.html");

    copy_assets();

    println!("SSG build complete.");
}

const INLINE_SCRIPT: &str = r##"
(function(){
// Hamburger menu
var h=document.getElementById('hamburger'),m=document.getElementById('mobileMenu');
if(h&&m){
h.addEventListener('click',function(){
  var o=m.style.display!=='flex';
  m.style.display=o?'flex':'none';
  h.className=o?'md:hidden flex flex-col justify-center w-6 h-[18px] relative hamburger active cursor-pointer':'md:hidden flex flex-col justify-center w-6 h-[18px] relative hamburger cursor-pointer';
  document.body.style.overflow=o?'hidden':'';
});
m.querySelectorAll('a').forEach(function(l){
  l.addEventListener('click',function(){m.style.display='none';document.body.style.overflow=''});
});
}
// Scroll reveal animations
var els=document.querySelectorAll('.bento-card,.timeline-item,.project-card,.contact-card,.section-lead');
if(els.length){
var obs=new IntersectionObserver(function(entries){
  entries.forEach(function(e){
    if(e.isIntersecting){e.target.classList.add('visible')}
  });
},{threshold:0.1,rootMargin:'0px 0px -40px 0px'});
els.forEach(function(el){el.classList.add('reveal');obs.observe(el)});
}
// Active nav tracking
var secs=document.querySelectorAll('section[id]'),links=document.querySelectorAll('.nav-links a[href^="#"]');
if(secs.length&&links.length){
var navObs=new IntersectionObserver(function(entries){
  entries.forEach(function(e){
    if(e.isIntersecting){
      links.forEach(function(l){l.classList.toggle('active',l.getAttribute('href')==='#'+e.target.id)});
    }
  });
},{threshold:0.3,rootMargin:'-80px 0px -50% 0px'});
secs.forEach(function(s){navObs.observe(s)});
}
// Nav scroll effect
var nav=document.getElementById('nav');
if(nav){
window.addEventListener('scroll',function(){nav.classList.toggle('scrolled',window.scrollY>50)});
}
})();
"##;

fn render_page() -> String {
    let mut vdom = VirtualDom::new(diegoquinas_rs::App);
    vdom.rebuild_in_place();

    let mut renderer = dioxus_ssr::Renderer::new();
    renderer.pre_render = true;
    let body = renderer.render(&vdom);

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Diego Perez Giordán - Backend Engineer</title>
    <meta name="description" content="Backend Engineer at Cencosud building Go microservices. Apache DataFusion open-source contributor in Rust." />
    <meta name="robots" content="index, follow" />
    <link rel="canonical" href="https://diegoquinas.com" />

    <meta property="og:type" content="website" />
    <meta property="og:title" content="Diego Perez Giordán - Backend Engineer" />
    <meta property="og:description" content="Backend Engineer at Cencosud building Go microservices. Apache DataFusion open-source contributor in Rust." />
    <meta property="og:url" content="https://diegoquinas.com" />
    <meta property="og:image" content="https://diegoquinas.github.io/og-image.png" />

    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@500;600&display=swap" rel="stylesheet" />
    <link rel="icon" type="image/svg+xml" href="assets/favicon.svg" />
    <link rel="stylesheet" href="assets/tailwind.css" />

    <script data-goatcounter="https://diegoquinas.goatcounter.com/count"
        async src="//gc.zgo.at/count.js"></script>
</head>
<body>
    <div id="main">{body}</div>
    <script>{INLINE_SCRIPT}</script>
    <script type="module" src="assets/diegoquinas-rs.js"></script>
</body>
</html>"#
    )
}

fn copy_assets() {
    let entries = fs::read_dir("assets").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            fs::copy(&path, format!("dist/assets/{filename}")).unwrap();
            println!("  dist/assets/{filename}");
        }
    }
}
