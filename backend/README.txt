docker Compose file: 
# https://docs.docker.com/compose/compose-file/

how to run docker:
Make sure you have docker installed
docker-compose up -d

how to run project:
cargo run


for postgres

#login into postgres todo DB
psql -h 127.0.0.1 -p 5432 -U todo todo

# for migration
psql -h 127.0.0.1 -p 5432 -U todo todo < database.sql

