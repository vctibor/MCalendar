# MCalendar

Self-hosted calendar following KISS principle, as minimalist as possible. Fullstack Rust application, using [seed](https://seed-rs.org/) for frontend and [actix-web](https://actix.rs/) for backend.

<img src="./screenshot.png?raw=true" style="box-shadow: 2px 2px 5px #151515;"/>

## Features

 - Assign single string to any date. Display all days in given month with their strings if any.

 - Weekend and holiday dates are highlighted with different color. Holidays for Czechia are fetched from third-party service. If there's no string associated with holiday date, holiday name is displayed in its place.

 - Highlight current day with different color.

## Setup

### Cargo

```sh
cargo install cargo-make
cargo install wasm-bindgen-cli
```

### Database

You need to have PostgreSQL database running on port 5342. Database setup:

Create user:

```sql
create user mcalendar with encrypted password 'mcalendar';
```

Create database `mcalendar`:

```sql
create database mcalendar with
    owner mcalendar
    encoding = 'UTF8'
    template template0;
```

Switch to newly created database and create table `events`:

```sql
create table events (
    date date not null,
    event text not null,
    primary key(date)
);
```

Grant user priviliges for newly created database:

```sql
grant all privileges on all tables in schema public to mcalendar;
```

### Local dev

Frontend is dependent on backend API being available, therefore we can't run it from `client` directory by itself.

Use `dev_run.sh` script to compile frontend, copy artifacts to backend and compile backend as well.

You need to re-run this script whenever you want to reload your changes.

You can use `build_release.sh` script to compile everthing using `release` configuration.

### Docker

To build Dockerfile on your local machine:

```
docker build -t mcalendar .
```

Start Docker image. It will listen on port 9000. You need to provide environment variable DATABASE_URL containing connection string to your database.

```sh
docker run \
    -p 9999:9000 \
    -e DATABASE_URL="postgresql://<user>:<password>@<server>:<port>" \
    mcalendar
```

## Technical details

Written in Rust, using Postgres database.

Licensed under terms of GPLv3 license.

See also [Rust fullstack single binary example](https://github.com/vctibor/seed_fullstack).
