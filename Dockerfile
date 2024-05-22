FROM rust:latest as base
RUN cargo install trunk
RUN apt-get update && apt-get install --yes libpq-dev nodejs npm

FROM base as frontend
WORKDIR /usr/app
COPY ./notes_r_us_ui/ ./notes_r_us_ui/
WORKDIR /usr/app/notes_r_us_ui/
RUN rustup target add wasm32-unknown-unknown
RUN npm install && trunk build

FROM base as api
WORKDIR /usr/app
COPY . .
RUN cargo build

FROM debian:latest as server
WORKDIR /usr/app
COPY --from=frontend /usr/app/notes_r_us_ui/dist ./notes_r_us_ui/dist
COPY --from=api usr/app/target/debug/notes_r_us ./notes_r_us
RUN ls
EXPOSE 3000
CMD ["/usr/app/notes_r_us"]


