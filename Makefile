.PHONY: all ssg wasm tailwind serve clean

WASM_TARGET = wasm32-unknown-unknown
RELEASE_DIR = target/$(WASM_TARGET)/release
OUT_DIR = dist

all: tailwind ssg wasm

tailwind:
	npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css

ssg:
	cargo run --bin ssg --release

wasm:
	cargo build --bin diegoquinas-rs --target $(WASM_TARGET) --features web --release
	wasm-bindgen $(RELEASE_DIR)/diegoquinas-rs.wasm \
		--out-dir $(OUT_DIR)/assets \
		--target web

serve:
	cd $(OUT_DIR) && python3 -m http.server 8000

clean:
	rm -rf $(OUT_DIR)
	cargo clean
