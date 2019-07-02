FROM rust:latest

RUN apt-get -y update
RUN apt-get -y install cmake libudev-dev yasm

COPY ./ /work
WORKDIR /work

RUN cargo build --release --features final