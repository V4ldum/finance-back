FROM rust:slim AS build
ENV SQLX_OFFLINE=true
WORKDIR /work
COPY . .

# Build
WORKDIR /work/sqlite3_unaccent
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --locked -q # Building sqlite extension

WORKDIR /work
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --locked -q


FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=build /work/target/release/finance-api .
COPY --from=build /work/sqlite3_unaccent/target/release/libsqlite3_unaccent.so /usr/lib/

EXPOSE 7878
ENTRYPOINT ["/app/finance-api"]
