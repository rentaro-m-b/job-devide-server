include .env
export

# appとdbのビルド
build:
	docker compose build

# 起動
up:
	docker compose up -d --wait --build
	sleep 5

# Dieselマイグレーション
migrate:
	docker compose exec app diesel migration run
	docker compose exec app diesel print-schema > src/schema.rs

all: build up migrate

.PHONY: build up migrate all
