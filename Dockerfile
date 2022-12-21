FROM rust:latest as build

WORKDIR /usr/src/mentalika-rs

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

COPY . .

RUN trunk build --release

FROM nginx

COPY --from=build /usr/src/mentalika-rs/dist /usr/share/nginx/html
RUN sed -i '9 a\        try_files $uri $uri/ /index.html;' /etc/nginx/conf.d/default.conf

ENTRYPOINT [ "nginx" ]
CMD ["-g", "daemon off;"]