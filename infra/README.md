## setup
assumes following files exist:
- `local.production.toml` at `service/fin_config/local.production.toml` (note local. files are ignore by git)
- `local.cf_secret` (look at example.cf_secret)

install roles
`make ansible-galaxy`

comment out role section in playbook.yml
  `make run-ansible`

un-comment out role section in playbook.yml
  `make run-ansible`


## backups
- create
`pg_dump r_fin > ~/Desktop/out.bak`
`pg_dump -h xxx -p 5432 -U xxx r_fin > 2020_nov_29.bak`

- restore
`psql -h fin-postgres.xxx.us-west-1.rds.amazonaws.com -p 5432 -U xxx r_fin < out.bak`



## postgres stuff
```
/etc/postgresql/11/main/pg_hba.conf

local   all             postgres                                peer
  should be:
local   all             postgres                                md5


```
`sudo service postgresql restart`

### run migrations without diesel
```
psql -U postgres  -d fin-prod -a -f migrations/00000000000000_diesel_initial_setup/up.sql
psql -U postgres  -d fin-prod -a -f migrations/2018-10-07-022941_init/up.sql
psql -U postgres  -d fin-prod -a -f migrations/2018-10-07-232226_initial_data/up.sql
psql -h xxx -p 5432 -U xxx r_fin -f migrations/2020-11-29-201034_add_growth_goal_portfolio/up.sql
```
