# Colors for better visibility in output
GREEN := \033[0;32m
BOLD := \033[1m
RESET := \033[0m

# Some setup variables.
POSTGRES_IMAGE=pgvector/pgvector:pg17
RABBITMQ_IMAGE=rabbitmq:latest

# Default configuration for local development.
SECRET_KEY=secretkey
BUCKET_NAME=dl-9799a9487ced
QUEUE_URL=amqp://localhost:5672/%2f
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
	@echo "RabbitMQ:"
	@echo "  - pull_rabbitmq: Pull RabbitMQ image from Docker Hub"
	@echo "  - run_rabbitmq: Run RabbitMQ Docker container"
	@echo "  - stop_rabbitmq: Stop the running RabbitMQ container"
	@echo ""
	@echo "Setup Scripts:"
	@echo "  - setup_server"
	@echo "  - setup_templates"
	@echo "  - setup_web"
	@echo "  - setup_extractor"

.PHONY: pull_postgres
pull_postgres:
	@echo "Pulling Postgres image from Docker Hub..."
	@docker pull $(POSTGRES_IMAGE)
	@docker tag $(POSTGRES_IMAGE) dl-postgres:latest
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

.PHONY: pull_rabbitmq
pull_rabbitmq:
	@echo "Pulling RabbitMQ image from Docker Hub..."
	@docker pull $(RABBITMQ_IMAGE)
	@docker tag $(RABBITMQ_IMAGE) dl-rabbitmq:latest
	@echo "$(GREEN)RabbitMQ image pulled successfully as:$(RESET)"
	@echo "dl-rabbitmq:latest"

.PHONY: run_rabbitmq
run_rabbitmq:
	@echo "Running RabbitMQ Docker container..."
	@docker run -d --name dl-rabbitmq \
	-p 5672:5672 \
	-p 15672:15672 \
	dl-rabbitmq:latest

	@echo "$(GREEN)RabbitMQ Docker container is running on:$(RESET)"
	@echo "$(QUEUE_URL)"

.PHONY: stop_rabbitmq
stop_rabbitmq:
	@echo "Stopping RabbitMQ Docker container..."
	@docker stop dl-rabbitmq
	@docker rm dl-rabbitmq
	@echo "$(GREEN)The RabbitMQ container is stopped and removed.$(RESET)"

.PHONY: setup_server
setup_server:
	@echo "Setting up server environment..."
	@$(MAKE) pull_postgres
	@$(MAKE) run_postgres

	@$(MAKE) pull_rabbitmq
	@$(MAKE) run_rabbitmq

	@touch server/.env
	@echo "DL_QUEUE_URL=$(QUEUE_URL)" > server/.env
	@echo "DL_DATABASE_URL=$(DATABASE_URL)" >> server/.env
	@echo "DL_SECRET_KEY=$(SECRET_KEY)" >> server/.env
	@echo "DL_BUCKET_NAME=$(BUCKET_NAME)" >> server/.env
	@echo "AWS_ACCESS_KEY_ID=xxx" >> server/.env
	@echo "AWS_SECRET_ACCESS_KEY=xxx" >> server/.env
	@echo "AWS_REGION=us-east-1" >> server/.env
	@echo "OPENAI_API_KEY=xxx" >> server/.env

	@cd server && cargo run migrate
	@echo "$(GREEN)Server environment setup complete:$(RESET)"
	@echo "Please provide the .env file with the correct values."

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
	@echo "DL_QUEUE_URL=$(QUEUE_URL)" >> extractor/.env
	@echo "DL_BUCKET_NAME=$(BUCKET_NAME)" >> extractor/.env
	@echo "AWS_ACCESS_KEY_ID=xxx" >> extractor/.env
	@echo "AWS_SECRET_ACCESS_KEY=xxx" >> extractor/.env
	@echo "AWS_REGION=us-east-1" >> extractor/.env

	@echo "$(GREEN)Extractor environment setup complete:$(RESET)"
	@echo "Please provide the .env file with the correct values."

.PHONY: teardown
teardown:
	@$(MAKE) stop_postgres
	@$(MAKE) stop_rabbitmq
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
