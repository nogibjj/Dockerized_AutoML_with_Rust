FROM rust:1.67

WORKDIR /usr/src/app
COPY auto/target/release/auto /usr/src/app

CMD ["app"]