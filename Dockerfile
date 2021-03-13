FROM vladimirmalky/rust_builder:latest AS build

RUN apt-get update && apt-get upgrade -y 

WORKDIR /mcalendar

ADD src src
ADD shared shared
ADD client client
ADD server server
ADD Cargo.toml Cargo.toml
ADD build_release.sh build_release.sh

RUN chmod o+x build_release.sh
RUN ./build_release.sh

FROM debian:buster AS run
COPY --from=build /mcalendar/target/release/mcalendar_server .

RUN apt-get update && apt-get install -y libssl1.1

ENTRYPOINT [ "/mcalendar_server" ]