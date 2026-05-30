.PHONY: blog blog-serve blog-clean

# Build the blog from blog-src/ into ./blog (committed and served by GitHub Pages).
blog:
	cd blog-src && zola build --output-dir ../blog --force

# Live-reload preview while writing posts.
blog-serve:
	cd blog-src && zola serve

# Remove the generated output.
blog-clean:
	rm -rf blog
