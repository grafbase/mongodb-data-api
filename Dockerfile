FROM rust:1.71
COPY . /usr/src/
WORKDIR "/usr/src"
RUN cargo build --release && mv target/release/mongodb-atlas-data-api /usr/local/ && rm -rf /usr/src
WORKDIR "/"
CMD ["/usr/local/mongodb-atlas-data-api", "--hostname", "0.0.0.0", "--port", "3000"]
