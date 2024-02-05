FROM rust:slim-buster

WORKDIR /app

COPY Cargo.* ./

COPY . .

EXPOSE 8000

CMD ["cargo", "run"]
