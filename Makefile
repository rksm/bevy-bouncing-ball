.PHONY: watch
watch:
	fd -E target -E pkg | entr -r make dev

.PHONY: dev
dev:
	RUST_BACKTRACE=1 cargo run --features bevy/dynamic

.PHONY: prod
prod:
	cargo run --release
