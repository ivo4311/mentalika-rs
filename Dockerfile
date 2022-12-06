FROM rust:latest as build

WORKDIR /usr/src/mentalika-rs

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

COPY . .

RUN trunk build --release

FROM nginx

COPY --from=build /usr/src/mentalika-rs/dist /usr/share/nginx/html

ENTRYPOINT [ "nginx" ]
CMD ["-g", "daemon off;"]