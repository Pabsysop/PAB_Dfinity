all: board anderson nais
.PHONY: all

board:
	cargo build --target wasm32-unknown-unknown --package board --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/board.wasm -o ./target/wasm32-unknown-unknown/release/board_opt.wasm

anderson:
	cargo build --target wasm32-unknown-unknown --package anderson --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/anderson.wasm -o ./target/wasm32-unknown-unknown/release/anderson_opt.wasm

nais:
	cargo build --target wasm32-unknown-unknown --package nais_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/nais_canister.wasm -o ./target/wasm32-unknown-unknown/release/nais_canister_opt.wasm

assets:
	cargo build --target wasm32-unknown-unknown --package Assets_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/Assets_canister.wasm -o ./target/wasm32-unknown-unknown/release/Assets_canister_opt.wasm
