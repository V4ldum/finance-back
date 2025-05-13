FROM rust:alpine AS build
ENV SQLX_OFFLINE=true
WORKDIR /work
COPY . .

RUN apk update
RUN apk upgrade --no-cache
RUN apk add --no-cache curl lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git make build-base bash curl wget zip gnupg coreutils gcc g++ zstd binutils ca-certificates upx

RUN cargo build --release --locked -q


FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app

COPY --from=build /work/target/release/finance-api finance-api

EXPOSE 7878
ENTRYPOINT ["/app/finance-api"]
