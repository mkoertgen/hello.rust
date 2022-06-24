FROM rust:1-slim-bullseye as builder
WORKDIR /usr/src/app
COPY . .
#RUN cargo test -r
RUN cargo install --path .
#RUN ls -hal /usr/local/cargo/bin/

FROM debian:bullseye-slim
ARG APP=hello-rust
# RUN apt-get update && apt-get install -y <extra-runtime-dependencies> && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP} /usr/local/bin/app
CMD ["app"]