FROM rust:1.77.2
LABEL authors="Fempty"

COPY ./ ./

RUN cargo build --release

EXPOSE 8000
EXPOSE 80
CMD ["./target/release/ServerByAxum"]

