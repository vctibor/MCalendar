# MCalendar

Web based calendar following KISS principle, as minimalist as possible.

![Screenshot](./screenshot.png?raw=true "Screenshot")

## Features

 - Assign single string to any date. Display all days in given month with their strings if any.

 - Weekend and holiday dates are highlighted with different color. Holidays for Czechia are fetched from third-party service. If there's no string associated with holiday date, holiday name is displayed in its place.

## TODO

 - Highlight current day with different color.

 - On Enter key write all strings (currently only selected row).

## Technical details

Written in Rust, using Postgres database.

Licensed under terms of GPLv3 license.
