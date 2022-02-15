FROM rust:1.52.1 as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/registration-server /
ADD static ./static
EXPOSE 8080
CMD ["./registration-server"]
