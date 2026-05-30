+++
title = "Building a blog the lazy-but-clean way"
date = 2026-05-29
description = "Why I bolted a Zola-generated blog onto a hand-crafted vanilla portfolio instead of rewriting everything."
[extra]
tags = ["meta", "rust", "zola"]
+++

This portfolio was vanilla HTML, CSS and JavaScript — no framework, no build
step, zero dependencies. I wanted a blog without throwing that away.

So the blog is a bolt-on: Zola generates `/blog` from Markdown and reuses the
portfolio's stylesheet. The home page stays hand-crafted.

A bit of code, to check that syntax highlighting works:

```rust
fn main() {
    let greeting = "Hello from the blog";
    println!("{greeting}");
}
```

And inline `code` renders with the accent color too.

That's the whole idea: write Markdown, run `make blog`, push.
