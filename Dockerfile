FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/bloom_filter_server /bin/bfserver

EXPOSE 1337
CMD ["bfserver"]
