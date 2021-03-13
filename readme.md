# MCalendar

Web based calendar following KISS principle, as minimalist as possible. Fullstack Rust application, using [seed](https://seed-rs.org/) for frontend and [actix-web](https://actix.rs/) for backend.


![Screenshot](./screenshot.png?raw=true "Screenshot")

## Features

 - Assign single string to any date. Display all days in given month with their strings if any.

 - Weekend and holiday dates are highlighted with different color. Holidays for Czechia are fetched from third-party service. If there's no string associated with holiday date, holiday name is displayed in its place.

## Setup

### Setup using Dockerfile

You have to build Dockerfile on your local machine.

You need to have PostgreSQL database running on port 5342. To change some values of connection string, such as port, username, password or database name, you will need to modify Dockerfile.

Create user:

    CREATE USER mcalendar WITH ENCRYPTED PASSWORD 'mcalendar';

Create database `mcalendar`:

    CREATE DATABASE mcalendar WITH
        OWNER mcalendar
        ENCODING = 'UTF8'
        TEMPLATE template0;

Switch to newly created database and create table `events`:

    CREATE TABLE Events (
        Date DATE NOT NULL,
        Event TEXT NOT NULL,
        PRIMARY KEY(Date)
    );

Grant user priviliges for newly created database:

    GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO mcalendar;

Start Docker image. By default it will listen on port 8000.

## TODO

 - Highlight current day with different color.

 - On Enter key write all strings (currently only selected row).

## Technical details

Written in Rust, using Postgres database.

Licensed under terms of GPLv3 license.

## Fullstack Rust web application

See also [Rust fullstack single binary example](https://github.com/vctibor/seed_fullstack).

### Client

Checkout [quickstart](https://github.com/seed-rs/seed-quickstart) and [counter example](https://github.com/seed-rs/seed/tree/master/examples/counter).

    cargo make watch

    cargo make serve

### Server

Based on https://github.com/seed-rs/seed/blob/master/examples/server_integration/server/src/main.rs.

### Build

In client:

    cargo make verify

    cargo make build_release