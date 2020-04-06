build:
	wasm-pack build --scope caolo-game

test:
	cargo check
	wasm-pack test --firefox --headless
	wasm-pack test --chrome --headless

pack:
	wasm-pack pack

publish: build
	cd pkg && npm publish --access=public
