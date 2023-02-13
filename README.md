# Axum Rest Service

## Project information

* Multi-module project
* Postgres
* Migrations
* Sqlx
* Redis - for working with jwt
* JWT
* Custom errors - AppGenericError
* Docker-compose

## Features

This project is aimed at showcasing how you could do:

* Project's layer structure - Controller, Service, Repository
* Cookie based server-side sessions
* Using Redis as a session storage mechanism
* Using Postgres connection pool
* Using Redis connection pool

## Entrypoint
 ```
  ./src/bin/servers.rs main 
 ```

## Run
 - docker containers
 ```
 $ cd distrbution/dev
 $ docker-compose up -d
 ```
- build and start app
 ```
 $ cargo build --release
 $ ./target/release/server
 ```
- for development
 ```
 $ cargo watch -x run
 ```