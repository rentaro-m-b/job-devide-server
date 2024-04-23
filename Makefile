include .env
export

# appとdbのビルド
build:
	docker compose build

# 起動
up:
	docker compose up -d --wait --build

# Dieselマイグレーション
migrate:
	docker compose exec app diesel migration run
	docker compose exec app diesel print-schema > src/schema.rs

all: build up

.PHONY: build up all
https://github.com/yumemi/bspoc-backend/blob/main/docker-compose.yml#L24-L29
