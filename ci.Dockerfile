# Have Allready Built The Frontend

FROM rust:latest as api
WORKDIR /usr/app
COPY . .
RUN cargo build

FROM debian:latest as server
WORKDIR /usr/app
COPY --from=api /usr/app/notes_r_us_ui/dist ./notes_r_us_ui/dist
COPY --from=api usr/app/target/debug/notes_r_us ./notes_r_us
EXPOSE 3000
CMD ["/usr/app/notes_r_us"]
