#!/bin/sh
SCRIPT_DIR=$(cd $(dirname $0); pwd)
DB_DIR=$SCRIPT_DIR/..

if ! command -v entr > /dev/null; then
    echo "entr がインストールされていません。インストールしてください"
    return 1
fi
find $DB_DIR/sql/stored/ -name '*.sql' | entr "$DB_DIR/bin/apply_procedure_local.sh"