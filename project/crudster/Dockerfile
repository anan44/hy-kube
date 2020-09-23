FROM rustlang/rust:nightly@sha256:63672289adce437e6383dd01d5f0ed8b3e33c30ebe583b2d198ecd56eb1db1de as builder
WORKDIR /builder/
COPY . /builder/
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app/
#RUN apt-get update && apt-get -y install openssl ca-certificates
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /builder/target/release/crudster /app/target/release/crudster
COPY ./frontend /app/frontend

CMD ["./target/release/crudster"]
