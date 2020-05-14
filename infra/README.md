
```
/etc/postgresql/11/main/pg_hba.conf

local   all             postgres                                peer
  should be:
local   all             postgres                                md5


```
`sudo service postgresql restart`

psql -U postgres  -d fin-prod -a -f migrations/00000000000000_diesel_initial_setup/up.sql
psql -U postgres  -d fin-prod -a -f migrations/2018-10-07-022941_init/up.sql
psql -U postgres  -d fin-prod -a -f migrations/2018-10-07-232226_fake_data/up.sql


## backups
create
`pg_dump r_fin > ~/Desktop/out.bak`
restore
`psql -h fin-postgres.xxx.us-west-1.rds.amazonaws.com -p 5432 -U xxx r_fin < out.bak`
