build:
	wasm-pack build --scope caolo-game

test:
	cargo check
	wasm-pack test --firefox --headless
	wasm-pack test --chrome --headless

testff:
	cargo check
	wasm-pack test --firefox --headless

pack:
	wasm-pack pack

publish: build
	cd pkg && npm publish --access=public
