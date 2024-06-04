FROM rust
COPY . /webcrawler
WORKDIR /webcrawler
CMD cargo run
