
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
