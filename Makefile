.PHONY: build up down clean

build:
	docker compose exec dev bash -c "cd snake-game && RUSTFLAGS='--cfg getrandom_backend=\"wasm_js\"' wasm-pack build --target web"

up:
	docker compose up -d

down:
	docker compose down

clean:
	rm -rf snake-game/pkg/
