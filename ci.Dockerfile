FROM debian:latest
WORKDIR /usr/app
COPY ./notes_r_us_ui/ ./notes_r_us_ui/dist
COPY ./notes_r_us/notes_r_us ./notes_r_us
RUN ls -la 
EXPOSE 3000
CMD ["/usr/app/notes_r_us"]
