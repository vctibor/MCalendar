FROM vladimirmalky/rust_builder:latest AS build

RUN apt-get update && apt-get upgrade -y 

WORKDIR /mcalendar

ADD shared shared
ADD client client
ADD server server
ADD Cargo.toml Cargo.toml
ADD build_and_run.sh build_and_run.sh

RUN chmod o+x build_and_run.sh
RUN ./build_and_run.sh


FROM debian:buster AS run
COPY --from=run target/debug/mcalendar_server .

RUN mcalendar_server