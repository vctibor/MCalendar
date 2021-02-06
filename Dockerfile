FROM vladimirmalky/rust_builder AS builder

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y openssl libssl-dev build-essential pkg-config

RUN mkdir /root/mcalendar
COPY ./Cargo.toml /root/mcalendar/Cargo.toml

RUN mkdir /root/mcalendar/src
COPY ./src/* /root/mcalendar/src/

RUN cd ~/mcalendar/ && ~/.cargo/bin/cargo build --release



FROM debian

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y openssl  

RUN mkdir /var/mcalendar/

COPY --from=builder /root/mcalendar/target/release/mcalendar /var/mcalendar/mcalendar

COPY templates /var/mcalendar/templates
COPY static /var/mcalendar/static

RUN echo 'address = "0.0.0.0"' >> /var/mcalendar/config.toml
RUN echo 'port = 8000' >> /var/mcalendar/config.toml
RUN echo 'conn_string = "postgres://mcalendar:mcalendar@192.168.196.23:5432/mcalendar"' >> /var/mcalendar/config.toml
RUN echo 'templates = "/var/mcalendar/templates"' >> /var/mcalendar/config.toml
RUN echo 'wwwroot = "/var/mcalendar/static"' >> /var/mcalendar/config.toml

EXPOSE 8000

CMD ["/var/mcalendar/mcalendar", "/var/mcalendar/config.toml"]
