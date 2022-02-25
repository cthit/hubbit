# hubbit2

## Setup project (DEV)
Steps for settings up the project
    1. Copy `.env.example` to `.env` and fill in with relevant information (see comments in the file).
    1. Run the docker compose file (`docker compose up`).
    1. Run `make create_gamma_client` to create a gamma client for hubbit2.
    1. If you want some mock data, run `make mock`

## General development 
Things to keep in mind during development.

### Database changes
When changing the database in any way (adding/modifying a migration) one should also update `sqlx-data.json` by running:
`cargo sqlx prepare -- --lib`

### Graphql UI auth
There is a graphql ui hosted by the backend from which one can run graphql queries etc.
This is hosted on `backend_address/api/graphql`.
For it to work with the authentication one needs to go to the settings and set "request.credentials" to "include"
