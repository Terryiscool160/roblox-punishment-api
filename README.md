# roblox-punishment-api

[![build status](https://github.com/Terryiscool160/roblox-punishment-api/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Terryiscool160/roblox-punishment-api/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/terryiscool160/roblox-punishment-api/status.svg)](https://deps.rs/repo/github/terryiscool160/roblox-punishment-api)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

###

this is a simple actix-web powered api with a diesel backend to allow for storage for timed roblox bans or any other sort of punishment type

you can find current [endpoints](ENDPOINTS.md) alongside the current [database schema](DATABASE.md)

## Running Locally

1. you can install rust by following the [official guide](https://www.rust-lang.org/tools/install)
2. run `cargo install diesel_cli`
3. run `diesel migration run` to setup the database
4. execute `cargo run`
5. the server will start (by default) on `http:://127.0.0.1:8080/` if you are using the default .env

# contributing

feel free to make a pull request with any changes you feel are fit
