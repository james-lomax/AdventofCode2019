# TODO: Turn this into a service which you can directly run a debugger through with vs code remote
FROM rustlang/rust:nightly
RUN apt update
RUN apt install -y gdb
ADD . /code
WORKDIR /code
RUN cargo build
