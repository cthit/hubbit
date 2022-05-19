# hubbit2

## Setup project (DEV)

Steps for settings up the project

1. Copy `.env.example` to `.env` and fill in with relevant information (see comments in the file)
2. Run the docker compose file (`docker compose up`)
3. Run `make setup` to initialize some data into hubbit and gamma. It might be a good idea to wait for the containers to start before running the setup, around 30 seconds should be enough

## General development

Things to keep in mind during development.

### Database changes

When changing the database in any way (adding/modifying a migration) one should also update `sqlx-data.json` by running `cargo sqlx prepare -- --lib` in the backend folder while the database running. 
If the command complains about missing `DATABASE_URL` copy the `backend/.env.example` to `backend/.env`.

### GraphQL UI

There is a graphql ui hosted by the backend from which one can run graphql queries etc.
The playground is reachable from the frontend and the backend, but for authentication purposes the frontend one should be used. It can be reached on `frontend_address/api/graphql`, which is `http://localhost:3000/api/graphql` by default.

__IMPORTANT!__ The playground does not send cookies to the backend by default, which are required for authentication.
To enable sending cookies go to the settings tab in the playground and change `request.credentials` to `include`.
