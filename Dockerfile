FROM rust:1.99.1 AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM ubuntu:26.04 

WORKDIR /app

COPY --from=build /app/target/release/axum-app /app/axum-app

EXPOSE 3000

ENTRYPOINT ["./axum-app"]

