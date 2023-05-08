FROM rust:1.61.0
WORKDIR /usr/src/twelve
COPY . .
RUN cargo install --path .
CMD ["twelve"]
