export DATABASE_URL=postgresql://warp-db-user:warp-db-pwd@localhost:5438/warp-db

dev-db:
	docker-compose -f ./repository/docker-compose.yaml up -d
	echo "Created Warp DB, remember to run db-migratoin"

db-migration:
	sqlx migrate --source ./repository/migrations run

clean-dev-db:
	docker-compose -f ./repository/docker-compose.yaml down
	rm -rf ./repository/.postgres-data

build:
	cargo build
