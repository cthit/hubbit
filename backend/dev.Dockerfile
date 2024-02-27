FROM rust:1.70

ENV TZ=Europe/Stockholm
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y tzdata postgresql-13

WORKDIR /app

RUN cargo install sqlx-cli
RUN cargo install cargo-watch

ENV PORT=8080
EXPOSE 8080

CMD cargo sqlx migrate run && cargo watch -x run
