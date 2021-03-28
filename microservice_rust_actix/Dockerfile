FROM rust:1-buster as base
ENV USER=root

WORKDIR /code
COPY docker.config $CARGO_HOME/config
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY for_docker_build_bin /code/for_docker_build_bin
RUN cargo build  --bin hello --release --offline

COPY src /code/src

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM  rust:1-buster

COPY --from=builder /code/target/release/microservice_rust_actix /usr/bin/microservice_rust_actix
COPY --from=builder /code/target/release/pq /usr/bin/pq

EXPOSE 5000

ENTRYPOINT [ "/usr/bin/microservice_rust_actix" ]
