# Colors for better visibility in output
GREEN := \033[0;32m
BOLD := \033[1m
RESET := \033[0m

# Default configuration for local development.
SECRET_KEY=secretkey
DATABASE_URL=postgres://postgres:password@localhost:5432/postgres
EXTRACTOR_HOST=0.0.0.0

.PHONY: all
all:
	@echo "DocuLens Makefile for Developers"
	@echo ""
	@echo "$(BOLD)Usage$(RESET): make <target>"
	@echo ""
	@echo "$(BOLD)Available Target$(RESET)"
	@echo "  - teardown: Teardown development environment"
	@echo "  - generate_rpc_stubs: Generate RPC stubs for clients"
	@echo ""
	@echo "Postgres:"
	@echo "  - pull_postgres: Pull Postgres image from Docker Hub"
	@echo "  - run_postgres: Run Postgres Docker container"
	@echo "  - stop_postgres: Stop the running Postgres container"
	@echo ""
	@echo "Setup Scripts:"
	@echo "  - setup_server"
	@echo "  - setup_templates"
	@echo "  - setup_web"
	@echo "  - setup_extractor"

.PHONY: pull_postgres
pull_postgres:
	@echo "Pulling Postgres image from Docker Hub..."
	@docker pull postgres:latest
	@docker tag postgres:latest dl-postgres:latest
	@echo "$(GREEN)Postgres image pulled successfully as:$(RESET)"
	@echo "dl-postgres:latest"

.PHONY: run_postgres
run_postgres:
	@echo "Running Postgres Docker container..."
	@docker run -d --name dl-postgres \
	-e POSTGRES_PASSWORD=password \
	-p 5432:5432 dl-postgres:latest

	@echo "$(GREEN)Postgres Docker container is running on:$(RESET)"
	@echo "$(DATABASE_URL)"

.PHONY: stop_postgres
stop_postgres:
	@echo "Stopping Postgres Docker container..."
	@docker stop dl-postgres
	@docker rm dl-postgres
	@echo "$(GREEN)Postgres Docker container stopped and removed.$(RESET)"

.PHONY: setup_server
setup_server:
	@echo "Setting up server environment..."
	@$(MAKE) pull_postgres
	@$(MAKE) run_postgres
	@touch server/.env
	@echo "DL_DATABASE_URL=$(DATABASE_URL)" > server/.env
	@echo "DL_SECRET_KEY=$(SECRET_KEY)" >> server/.env
	@echo "$(GREEN)Server environment setup complete.$(RESET)"

.PHONY: setup_templates
setup_templates:
	@echo "Setting up templates directory..."
	@cd templates && \
	python3 -m venv .venv && \
	source .venv/bin/activate && \
	pip install -r requirements.txt

	@echo "$(GREEN)Templates environment setup complete.$(RESET)"

.PHONY: setup_web
setup_web:
	@echo "Setting up web application environment..."
	@cd web && \
	npm install

	@cp web/.env.example web/.env
	@echo "$(GREEN)Web environment setup complete:$(RESET)"
	@echo "Please update the .env file with the correct values."

.PHONY: setup_extractor
setup_extractor:
	@echo "Setting up extractor environment..."
	@cd extractor && \
	poetry config virtualenvs.in-project true && \
	poetry install

	@touch extractor/.env
	@echo "DL_EXTRACTOR_HOST=$(EXTRACTOR_HOST)" > extractor/.env
	@echo "$(GREEN)Extractor environment setup complete.$(RESET)"

.PHONY: teardown
teardown:
	@$(MAKE) stop_postgres
	@echo "$(GREEN)Environment teardown complete.$(RESET)"

.PHONY: generate_rpc_stubs
generate_rpc_stubs:
	@echo "Generating RPC stubs..."
	@python -m pip install grpcio-tools
	@mkdir -p extractor/src/stubs
	@touch extractor/src/stubs/__init__.py
	@python -m grpc_tools.protoc -I./server/protos \
	--python_out=./extractor/src/stubs \
	--grpc_python_out=./extractor/src/stubs \
	./server/protos/*.proto

	@echo "$(GREEN)RPC stubs generated.$(RESET)"
