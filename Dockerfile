FROM rust:1.58 as build-env
WORKDIR /
ADD backend/ /backend
ADD frontend/ /frontend
RUN cd backend && cargo build --release
RUN rustup target add wasm32-unknown-unknown && cargo install trunk
RUN cd /frontend && trunk build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /backend/target/release/registration-server /
COPY --from=build-env /frontend/dist /static
EXPOSE 8080
CMD ["./registration-server"]
