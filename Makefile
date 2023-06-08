run:
	mkdir -p ./volumes/postgres_data &&\
	docker compose down --remove-orphans &&\
	docker compose up -d --build --force-recreate

run_migrations:
	DATABASE_URL="postgres://dominux_chat:lmao@localhost:5432/dominux_chat" \
	sea-orm-cli migrate refresh -d ./backend/libs/migration &&\
	DATABASE_URL="postgres://dominux_chat:lmao@localhost:5432/dominux_chat" \
	sea-orm-cli generate entity -o ./backend/libs/entity/src --with-serde both &&\
	rm ./backend/libs/entity/src/mod.rs

revert_migrations:
	DATABASE_URL="postgres://dominux_chat:lmao@localhost:5432/dominux_chat" \
	sea-orm-cli migrate reset -d ./backend/libs/migration
