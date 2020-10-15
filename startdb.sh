#!/bin/sh
./stopdb.sh
sleep 2
podman run --rm -e 'ACCEPT_EULA=Y' -e 'MSSQL_SA_PASSWORD=12345Abcde' --name 'mms-sqlserver' -d -p 1433:1433 mcr.microsoft.com/mssql/rhel/server
podman cp sql/mmsdm_tables.sql mms-sqlserver:/tmp/mmsdm_tables.sql
podman cp sql/mmsdm_procs.sql mms-sqlserver:/tmp/mmsdm_procs.sql
sleep 15
podman exec -it mms-sqlserver /opt/mssql-tools/bin/sqlcmd -U sa -P 12345Abcde -Q "create database mmsdm"
podman exec -it mms-sqlserver /opt/mssql-tools/bin/sqlcmd -U sa -P 12345Abcde -i /tmp/mmsdm_tables.sql
podman exec -it mms-sqlserver /opt/mssql-tools/bin/sqlcmd -U sa -P 12345Abcde -i /tmp/mmsdm_procs.sql