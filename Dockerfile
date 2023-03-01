#FROM rust:latest

#WORKDIR /usr/src/app

#COPY auto/ .

#RUN cargo build --release

#CMD ./target/release/auto

#FROM scratch
#COPY /auto/target/release/auto /auto
#CMD [./auto]


FROM rust:latest as builder
#ENV APP auto
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP/target/release/auto
#export this actix web service to port 8080 and 0.0.0.0
CMD [cargo run --release]