FROM messense/rust-musl-cross:aarch64-musl AS build
ENV SQLX_OFFLINE=true
WORKDIR /api
COPY . .
RUN cargo build --release --locked -q --target aarch64-unknown-linux-musl

FROM scratch
WORKDIR /api
COPY --from=build /api/target/aarch64-unknown-linux-musl/release/finance-api finance-api
ENTRYPOINT ["./finance-api"]
EXPOSE 7878