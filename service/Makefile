DB_URL='postgres://localhost/r_fin'
VERSION=0.1.1
NAME=fin
REGISTRY=toidiu
HOST=fin.toidiu.com

releaseLinuxExec:
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder:1.35.0-openssl11 cargo build --release && \
	scp target/x86_64-unknown-linux-musl/release/fin toidiu@${HOST}:service/fin && \
	scp -r fin_config/default.toml toidiu@${HOST}:service/fin_config && \
	scp -r fin_config/local.production.toml toidiu@${HOST}:service/fin_config/production.toml && \
	scp -r migrations toidiu@${HOST}:service/

releaseLinuxDocker:
	docker build -t $(NAME):$(VERSION) .

migrationRun:
	export DATABASE_URL=${DB_URL}; diesel migration run

migrationRedo:
	export DATABASE_URL=${DB_URL}; diesel migration redo

migrationRevert:
	export DATABASE_URL=${DB_URL}; diesel migration revert

migrationList:
	export DATABASE_URL=${DB_URL}; diesel migration list

migrationZZZ_DANGER_RESET_DB_ZZZ: #DANGER!!
	export DATABASE_URL=${DB_URL}; diesel database reset

migration_teardown_db:
	psql -U postgres -a -f teardown.sql;

.PHONY: migrationRun migrationRedo migrationRevert migrationList migrationZZZ_DANGER_RESET_DB_ZZZ releaseLinuxDocker releaseLinuxExec
