# Colors for better visibility in output
GREEN := \033[0;32m
BOLD := \033[1m
RESET := \033[0m

# Default configuration for local development.
DATABASE_URL=postgres://postgres:password@localhost:5432/postgres

.PHONY: all
all:
	@echo "DocuLens Makefile for Developers"
	@echo ""
	@echo "$(BOLD)Usage$(RESET): make <target>"
	@echo ""
	@echo "$(BOLD)Available Target$(RESET)"
	@echo "  - setup: Setup development environment"
	@echo "  - teardown: Teardown development environment"
	@echo "  - pull_postgres: Pull Postgres Docker image"
	@echo "  - run_postgres: Run Postgres Docker container"
	@echo "  - stop_postgres: Stop Postgres Docker container"

.PHONY: pull_postgres
pull_postgres:
	@echo "Pulling Postgres Docker image..."
	@docker pull postgres:latest
	@docker tag postgres:latest dl-postgres:latest
	@echo "$(GREEN)Postgres Docker image pulled successfully.$(RESET)"

.PHONY: run_postgres
run_postgres:
	@echo "Running Postgres Docker container..."
	@docker run -d --name dl-postgres \
	-e POSTGRES_PASSWORD=password \
	-p 5432:5432 dl-postgres:latest

	@echo "$(GREEN)Postgres Docker container is running:$(RESET)"
	@echo "$(DATABASE_URL)"

.PHONY: stop_postgres
stop_postgres:
	@echo "Stopping Postgres Docker container..."
	@docker stop dl-postgres
	@docker rm dl-postgres
	@echo "$(GREEN)Postgres Docker container stopped and removed.$(RESET)"

.PHONY: setup
setup:
	@$(MAKE) pull_postgres
	@$(MAKE) run_postgres

	@touch server/.env
	@echo "DL_DATABASE_URL=$(DATABASE_URL)" > server/.env

	@echo "$(GREEN)Environment setup complete.$(RESET)"

.PHONY: teardown
teardown:
	@$(MAKE) stop_postgres
	@echo "$(GREEN)Environment teardown complete.$(RESET)"
