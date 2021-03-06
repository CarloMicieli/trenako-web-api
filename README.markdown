# Trenako Web Api

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/tterb/atomic-design-ui/blob/master/LICENSEs)
![GitHub last commit](https://img.shields.io/github/last-commit/CarloMicieli/trenako-web-api)
![GitHub top language](https://img.shields.io/github/languages/top/CarloMicieli/trenako-web-api)
[![dev](https://github.com/CarloMicieli/trenako-web-api/actions/workflows/dev.yml/badge.svg)](https://github.com/CarloMicieli/trenako-web-api/actions/workflows/dev.yml)
[![main](https://github.com/CarloMicieli/trenako-web-api/actions/workflows/main.yml/badge.svg)](https://github.com/CarloMicieli/trenako-web-api/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/CarloMicieli/trenako-web-api/branch/main/graph/badge.svg?token=D01ZSPN6UR)](https://codecov.io/gh/CarloMicieli/trenako-web-api)
![Docker Image Version (tag latest semver)](https://img.shields.io/docker/v/carlomicieli/trenako-web-api/latest?style=plastic)

A web api for model railway collections

## Tech Stack

* 🦀 `Rust` 1.59
* `Cargo`
* `Docker` / `Docker compose`

## Run Locally

Clone the project

```bash
  git clone https://github.com/CarloMicieli/trenako-web-api.git
```

Go to the project directory

```bash
  cd trenako-web-api
```

## Project layout

* `db`: database migrations
* `openapi`: open api 3.0 schema for the web api
* `sources`: the service sources

## Installation 

To begin install `rustup`

```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
  cargo install cargo-tarpaulin
  cargo install cargo-audit
  cargo install sqlx-cli --no-default-features --features postgres
  rustup component add clippy
  rustup component add rustfmt
```

### Cargo Commands

|        Command                  | Description                |
|---------------------------------|----------------------------|
|`cargo run`                      |run the app                 |
|`cargo test`                     |run the tests               |
|`cargo fmt -- --check`           |check the formatting        |
|`cargo clippy`                   |run the linter              |
|`cargo tarpaulin --ignore-tests` |compute code coverage       |
|`cargo audit`                    |check for security warnings |

### Docker image

To build the docker image:

```bash
  docker build -t trenako-web-api:latest .
```

To run a `postgres` database:

```bash
  docker run -it --rm -p:5432:5432 \
    -e POSTGRES_PASSWORD=mysecretpassword \
    -e POSTGRES_DB=trenakodb \
    postgres
```

To run the server:

```bash
  export HOST_IP=<host_ip>
  docker run --rm \
    -e SERVER_HOST=0.0.0.0 \
    -e SERVER_PORT=5000 \
    -e DATABASE_NAME=trenakodb \
    -e DATABASE_USERNAME=postgres \
    -e DATABASE_PASSWORD=mysecretpassword \
    -e DATABASE_HOST=$HOST_IP \
    -e DATABASE_PORT=5432 \
    -p 5000:5000 \
    trenako-web-api:latest
```

### Database

To run the migrations:

```bash
  docker run --rm -it --network=host \
      -v "$(pwd)/db:/db" \
      amacneil/dbmate \
      --url "postgresql://postgres:mysecretpassword@<host_ip>:5432/trenako?sslmode=disable" \
      up
```

### OpenApi

```bash
  docker run --rm -p 8080:8080 \
    -e API_URL=doc/trenako-open-api.yaml \
    -v ${PWD}/openapi:/usr/share/nginx/html/doc \
    swaggerapi/swagger-ui
```

## Deployment

## API Reference

```bash
  pip install -U httpie
  pip install -U httpie-jwt-auth
```

#### Get all items

```http
  GET /api/items
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `api_key` | `string` | **Required**. Your API key |
  
## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.
  
### Conventional commits

This repository is following the conventional commits practice.

#### Enforcing using git hooks

```bash
  git config core.hooksPath .githooks
```

The hook itself can be found in `.githooks/commit-msg`.

#### Using Commitizen

Install [commitizen](https://github.com/commitizen-tools/commitizen)

```bash
  pip install commitizen
```

and then just use it

```bash
  cz commit
```

## License

[MIT License](https://choosealicense.com/licenses/mit/)

```
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
