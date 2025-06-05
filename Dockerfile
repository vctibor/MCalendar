FROM debian:latest AS build

RUN apt-get update

RUN apt-get upgrade -y

RUN apt-get install curl build-essential git pkg-config libssl-dev libpq-dev -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install_rustup.sh

RUN chmod o+x install_rustup.sh

RUN ./install_rustup.sh -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup -V
RUN rustc -V
RUN cargo -V

RUN cargo install cargo-deb

# faciliate building Seed applications
# https://github.com/seed-rs/seed-quickstart
RUN cargo install cargo-make

RUN apt-get update && apt-get upgrade -y 

WORKDIR /mcalendar

ADD src src
ADD shared shared
ADD client client
ADD server server
ADD utils utils
ADD Cargo.toml Cargo.toml
ADD build_release.sh build_release.sh

RUN chmod o+x build_release.sh
RUN ./build_release.sh

FROM debian:latest AS run
COPY --from=build /mcalendar/target/release/mcalendar_server .

RUN apt-get update && apt-get install -y libssl3

ENTRYPOINT [ "/mcalendar_server" ]