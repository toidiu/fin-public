DB_URL='postgres://localhost/r-fin'

run:
	export DATABASE_URL=${DB_URL}; diesel migration run

redo:
	export DATABASE_URL=${DB_URL}; diesel migration redo

revert:
	export DATABASE_URL=${DB_URL}; diesel migration revert

list:
	export DATABASE_URL=${DB_URL}; diesel migration list

ZZZ_DANGER_RESET_DB_ZZZ: #DANGER!!
	export DATABASE_URL=${DB_URL}; diesel database reset


.PHONY: run redo revert list ZZZ_DANGER_RESET_DB_ZZZ
