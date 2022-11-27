FROM rust:1.65.0

WORKDIR /app

COPY . /app

RUN cargo install --path .

#ENV DISCORD_TOKEN=token
#ENV MONGO_HOST=host
#ENV MONGO_PORT=port
#ENV API_HOST=host
#ENV API_PORT=port

ENTRYPOINT ["drunk-venti-rust"]
