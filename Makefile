include .env
export

# appとdbのビルド
build:
	docker compose build

# 起動
up:
	docker compose up -d --wait
	sleep 5

# Dieselマイグレーション
migrate:
	docker compose exec app diesel migration run

all: build up migrate

.PHONY: build up migrate all
