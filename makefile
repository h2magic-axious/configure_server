include .env
migrate:
	DATABASE_URL=${DATABASE_URL} sqlx migrate run
build:
	CROSS_COMPILE=x86_64-linux-musl cargo build --release --target x86_64-unknown-linux-musl