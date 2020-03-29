build:
	wasm-pack build --scope caolo-game

test:
	wasm-pack test

pack:
	wasm-pack pack

publish: build
	cd pkg && npm publish --access=public

