.PHONY: up bash logs down

up:
	docker-compose up -d

bash:
	docker-compose exec rust bash

logs:
	docker container logs -f aworld-server-rust

down:
	docker-compose down
