FROM rustlang/rust:nightly@sha256:63672289adce437e6383dd01d5f0ed8b3e33c30ebe583b2d198ecd56eb1db1de as builder
WORKDIR /builder/
COPY . /builder/
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app/
COPY --from=builder /builder/target/release/ /app/
CMD ["./uuid-writer"]
