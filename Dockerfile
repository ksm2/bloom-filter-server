FROM ubuntu:latest

COPY target/release/bloom_filter_server /srv/bloom_filter_server

EXPOSE 1337
CMD /srv/bloom_filter_server
