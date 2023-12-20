FROM rust:1.74-slim-bookworm

RUN apt update -y && \
    apt install protobuf-compiler -y

COPY . /root/grpc-server

WORKDIR /root/grpc-server

CMD ["cargo", "run"]
