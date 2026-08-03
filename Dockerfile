## CARGO CHEF ##
FROM lukemathwalker/cargo-chef:latest-rust-slim AS chef
WORKDIR /work


## PLAN ##
FROM chef AS plan
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


## BUILD ##
FROM chef AS build
ENV SQLX_OFFLINE=true
COPY --from=plan /work/recipe.json recipe.json

# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu -p api -p update-agent


## TOOLS ##
FROM busybox:musl AS tools
RUN mkdir -p /tools && \
    cp /bin/busybox /tools/ && \
    cd /tools && \
    ln -s busybox sh && \
    ln -s busybox test && \
    ln -s busybox touch


## RUN ##
FROM gcr.io/distroless/cc-debian13:nonroot
WORKDIR /app

COPY --from=tools /tools/ /bin/
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/api .
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/update-agent .

EXPOSE 7878
ENTRYPOINT ["/app/api"]
