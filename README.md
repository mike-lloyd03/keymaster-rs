# KeyMaster
## Key Inventory Manager

Thing for keeping track of who has what keys.

This is a Rust rewrite of [KeyMaster](https://github.com/mike-lloyd03/keymaster) which I wrote in Python a few years ago. The purpose of this project is to gain more experience developing in Rust as well as learning about web assembly and remembering everything I forgot about web dev.

## The Plan
### Backend
Develop a REST API which connects to a Postgres database for managing all the CRUD operations.

Crates:
- `sqlx` For database connection
- `actix-web` For backend web server and routing
- `actix-session` For session management
- `orion` For password hashing and validation

### Frontend
Develop a responsive, _BLAZINGLY FAST_ front end using modern technologies like web-assembly.

Crates:
- `yew` It's like React but on Rust steroids and compiles to wasm
- `yewdux` Awesome global state library akin to Redux


## Progress so Far
It's basically done. The app works and it works pretty good. There's a few quirks here and there that need to be cleaned up but I'll get around to it.

## Deployment
Clone the repo and do the following:

```
docker build . --tag keymaster:latest
```

Tweak the included `docker-compose.yml` file as you see fit. The program will initialize an admin user on startup if one doesn't already exist in the database. The admin password will be randomly generated and will be printed to the logs. Alternatively, you can set the following environment variables to configure them before running.

```
KEYMASTER_ADMIN_USER
KEYMASTER_ADMIN_PASS
```

This will have no effect after the admin user is initialized so you can delete them from your config after the database is up.

Once all that is done:

```
docker compose up -d
```

Navigate to `localhost:8080` (or whatever port you mapped) and have a look.

Todo:
- The UI is a bit dated. Port it over to tailwind.
