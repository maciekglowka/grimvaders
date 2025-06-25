FROM rust:1.86-bullseye

RUN rustup update
RUN apt-get update && apt-get install libasound2-dev -yy

WORKDIR /app

CMD ["cargo", "build", "--release"]
