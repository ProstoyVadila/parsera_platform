up:
	SCHEDULER_PORT=8001 docker compose -f test-compose.yaml up -d --build

down:
	docker compose down


up_release:
	SCHEDULER_PORT=8001 docker compose up -d --build


.PHONY: up up_release down
