# Colors for better visibility in output
GREEN := \033[0;32m
BOLD := \033[1m
RESET := \033[0m

.PHONY: all
all:
	@echo "DocuLens Makefile for Developers"
	@echo ""
	@echo "$(BOLD)Usage$(RESET): make <target>"
	@echo ""
	@echo "$(BOLD)Available Target$(RESET)"
	@echo "  - pull_postgres: Pull Postgres Docker image"
	@echo "  - run_postgres: Run Postgres Docker container"

.PHONY: pull_postgres
pull_postgres:
	@echo "Pulling Postgres Docker image..."
	@docker pull postgres:latest
	@docker tag postgres:latest dl-postgres:latest
	@echo "$(GREEN)Postgres Docker image pulled successfully!$(RESET)"

.PHONY: run_postgres
run_postgres:
	@echo "Running Postgres Docker container..."
	@docker run -d --name dl-postgres \
	-e POSTGRES_PASSWORD=password \
	-p 5432:5432 dl-postgres:latest

	@echo "$(GREEN)Postgres Docker container is running:$(RESET)"
	@echo "postgres://postgres:password@localhost:5432/postgres"
