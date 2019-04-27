FROM ubuntu

COPY mcalendar mcalendar

COPY mcalendar_config.toml mcalendar_config.toml

COPY mcalendar.sled mcalendar.sled

RUN chmod +x mcalendar

EXPOSE 8000

CMD ["./mcalendar"]