COMPOSE = docker compose

.PHONY: up down reset logs status shell admin gui-test gui-test-all gui-test-core gui-test-chat gui-test-ui

## Start the XMPP test server (detached).
up:
	$(COMPOSE) up -d
	@echo ""
	@echo "Waiting for Prosody to become healthy..."
	@for i in $$(seq 1 30); do \
		STATUS=$$(docker inspect --format='{{.State.Health.Status}}' rexisce-prosody 2>/dev/null); \
		if [ "$$STATUS" = "healthy" ]; then \
			echo "Prosody is ready."; \
			break; \
		fi; \
		sleep 1; \
	done
	@echo ""
	@echo "Connect with: alice@localhost / alice123 on localhost:5222"

## Stop the XMPP test server (preserves data).
down:
	$(COMPOSE) down

## Full reset: stop, remove volumes, rebuild.
reset:
	$(COMPOSE) down -v
	$(COMPOSE) up -d
	@echo "Server reset complete. All data wiped, accounts re-created."

## Tail Prosody logs.
logs:
	$(COMPOSE) logs -f prosody

## Show container status and health.
status:
	@docker inspect --format='Status: {{.State.Health.Status}} | Running: {{.State.Running}} | Uptime: {{.State.StartedAt}}' rexisce-prosody 2>/dev/null || echo "Container not running."

## Open a shell inside the Prosody container.
shell:
	docker exec -it rexisce-prosody bash

## Open Prosody's interactive admin shell.
admin:
	docker exec -it rexisce-prosody prosodyctl shell

# ---- GUI Tests (Claude Computer Use) ----

VENV = source ../.venv/bin/activate &&

## Run all GUI tests.
gui-test-all: up
	$(VENV) python gui_test.py --all

## Run core tests (login, wrong password).
gui-test-core: up
	$(VENV) python gui_test.py --group core

## Run chat tests (messaging, OMEMO, MUC).
gui-test-chat: up
	$(VENV) python gui_test.py --group chat

## Run UI tests (settings, theme, close conversation).
gui-test-ui: up
	$(VENV) python gui_test.py --group ui

## Run a single GUI test (usage: make gui-test TEST=login).
gui-test: up
	$(VENV) python gui_test.py $(TEST)

## List all available GUI tests.
gui-test-list:
	@$(VENV) python gui_test.py --list
