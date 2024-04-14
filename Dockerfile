FROM rust:latest

WORKDIR /app
COPY . .

# Diesel CLIをインストール
RUN cargo install diesel_cli --no-default-features --features postgres

# 環境変数の設定
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/job-devide-server"]
