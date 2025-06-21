FROM rust:slim AS build
ENV SQLX_OFFLINE=true
WORKDIR /work
COPY . .

# Build
WORKDIR /work/sqlite3_unaccent
RUN cargo build --release --locked --target aarch64-unknown-linux-gnu # Building sqlite extension

WORKDIR /work
RUN cargo build --release --locked --target aarch64-unknown-linux-gnu


FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=build /work/target/aarch64-unknown-linux-gnu/release/finance-api .
COPY --from=build /work/sqlite3_unaccent/target/aarch64-unknown-linux-gnu/release/libsqlite3_unaccent.so /usr/lib/

EXPOSE 7878
ENTRYPOINT ["/app/finance-api"]
