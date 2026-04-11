FROM rust:slim AS build
ENV SQLX_OFFLINE=true
WORKDIR /work
COPY . .

# Build
WORKDIR /work/sqlite3_unaccent
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu # Building sqlite extension

WORKDIR /work
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu -p api -p update-agent


FROM busybox:musl AS tools
RUN mkdir -p /tools && \
    cp /bin/busybox /tools/ && \
    cd /tools && \
    ln -s busybox sh && \
    ln -s busybox test && \
    ln -s busybox touch


FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=tools /tools/ /bin/
COPY --from=build /work/sqlite3_unaccent/target/x86_64-unknown-linux-gnu/release/libsqlite3_unaccent.so /usr/lib/
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/api .
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/update-agent .

EXPOSE 7878
ENTRYPOINT ["/app/api"]
