# aurion_rs

[![Crates.io](https://img.shields.io/crates/v/aurion_rs)](https://crates.io/crates/aurion_rs)
[![docs.rs](https://img.shields.io/docsrs/aurion_rs)](https://docs.rs/aurion_rs)
[![GitHub repo size](https://img.shields.io/github/repo-size/sehnryr/aurion_rs)](https://github.com/sehnryr/aurion_rs)
[![GitHub](https://img.shields.io/github/license/sehnryr/aurion_rs)](https://github.com/sehnryr/aurion_rs/blob/main/LICENSE)

Aurion is an ERP (Enterprise Resource Planning) developed by [Auriga](https://www.auriga.fr/)
used by multiple French universities. It is a web application that provides
a lot of services to students and teachers (timetable, grades, etc.).

This project aims to provide a Rust library to interact with Aurion.

This library is based on [isen_aurion_client](https://github.com/sehnryr/isen_aurion_client),
a Dart library that I wrote for my own needs. I decided to rewrite it in Rust
because I wanted to learn the language and I wanted to have a library that
could be used in a CLI application.

## Features

- [x] Login
- [x] Get the user's schedule
- [ ] Get a group schedule
- [ ] Get the user's grades
- [ ] Get the user's absences
- [ ] Get the user's registration certificate
- [ ] Get the user's school reports
