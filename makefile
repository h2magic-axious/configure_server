include .env
migrate:
	DATABASE_URL=$DATABASE_URL sqlx run