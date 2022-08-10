# KeyMaster
## Key Inventory Manager

Thing for keeping track of who has what keys.

This is a Rust rewrite of [KeyMaster](https://github.com/mike-lloyd03/keymaster) which I wrote in Python a few years ago. The purpose of this project is to gain more experience developing in Rust as well as learning some core Rust crates.

## Current Plan
### Backend
Develop a REST API which connects to a Postgres database for managing all the CRUD operations.

Crates:
- `sqlx` For database connection
- `actix-web` For backend web server and routing
- `actix-session` For session management
- `orion` For password hashing and validation

### Frontend
Develop a responsive _BLAZINGLY FAST_ front end using modern technologies like web-assembly.

Crates:
- `yew` It's like React but on Rust steroids and compiles to wasm


## Progress so Far
### Backend
The backend is nearly done. The following features are implemented:

- Actix powered web server which handles routes for CRUD ops on `Users`, `Keys`, and key `Assignments`
- Cookie-based session management
- Password hashing via Argon2
- Automatic admin user generation if the user does not already exist.
- All `POST` and `PUT` routes require admin privileges. All other routes require authentication.
- Tests for all model methods

To do:
- Write integration tests for each of the endpoints

### Frontend
The very barebones of the pages exist and are copies of the Jinja templates I had in place for the Python version of this.

- Learn `yew`
- Convert Jinja templates to yew components
- Figure out how to employ the backend's session management in the front end


We'll get there.
