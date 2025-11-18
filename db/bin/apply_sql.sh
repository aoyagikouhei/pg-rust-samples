#!/bin/sh
SCRIPT_DIR=$(cd $(dirname $0); pwd)
SQL_DIR=$SCRIPT_DIR/../sql

cat $SQL_DIR/01_prepare.sql > ./all.sql
cat $SQL_DIR/02_kbn.sql >> ./all.sql
cat $SQL_DIR/03_lib.sql >> ./all.sql
cat $SQL_DIR/table/*.sql >> ./all.sql
cat $SQL_DIR/stored/*.sql >> ./all.sql
cat $SQL_DIR/04_data.sql >> ./all.sql

psql --set "ON_ERROR_STOP=1" -p ${PGPORT:-5432} -h ${PGHOST:-localhost} -U ${PGUSER:-user} ${PGDATABASE:-web} -f ./all.sql

rm ./all.sql
