#!/bin/sh
# Can use `--pod=name` to run in concert with apps that depend on the db
# Can use `--net slirp4nets:port_handler=slirp4nets` to publish ports in rootless env
# Will need to run `podman secret create pg_pass` before running this script

podman run --rm -d \
	--name postgres \
	--secret pg_pass \
	-e POSTGRES_USER=user \
	-e POSTGRES_PASSWORD_FILE=/run/secrets/pg_pass \
	-e POSGRES_DB=load \
	-p 5432:5432 \
	-v "$PWD/postgresql.conf":/etc/postgresql/postgresql.conf \
	-v "$PWD/pgdata":/var/lib/postgresql/data \
	docker.io/postgres:latest
	
