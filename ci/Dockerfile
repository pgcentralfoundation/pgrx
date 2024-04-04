FROM rust:1

ARG PGRX_VERSION=0.12.0-alpha.1

RUN apt-get update && apt-get install -y \
    gettext-base \
    libclang-dev \
    postgresql-server-dev-all

USER postgres

ENV USER=postgres

ENV PATH="${PATH}:/usr/local/cargo/bin/:~postgres/.cargo/bin"

RUN rustup component add clippy && \
    cargo install --locked --version "${PGRX_VERSION}" cargo-pgrx && \
    cargo pgrx init

WORKDIR /pgrx

VOLUME /pgrx
