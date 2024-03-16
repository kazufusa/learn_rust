db-init:
	docker stop postgresql-container
	docker run --rm -d --name postgresql-container -p 5432:5432 -e POSTGRES_PASSWORD=password postgres

db-setup:
	PGPASSWORD=password psql -h localhost -U postgres -d postgres < preparation.sql

run:
	cargo run

db-show:
	PGPASSWORD=password psql -h localhost -U postgres -d postgres -c "SELECT * FROM users;"
