FROM rust:slim-buster as build

WORKDIR /repo

COPY dummy.rs .
COPY Cargo.lock .
COPY Cargo.toml .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo build --release \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY src src

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /repo/target/release/app /

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/app"]