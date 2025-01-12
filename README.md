![DocuLens](https://doculens-assets.s3.amazonaws.com/banners/logo.png)

DocuLens is a search API platform that allows users to build a retrieval system
for their own or their users' PDF documents. Some common use cases of DocuLens
are search engines for academic research papers or internal knowledge base
chatbots. At a high level, we have two main workflows in our system:
**Ingestion** and **Retrieval**.

This repository is a monorepo that contains the source code for our internal
components. Before starting with the development, you might want to set up your
local environment and install the necessary high-level dependencies such as:

- [Node.js](https://nodejs.org/en/download/package-manager)
- [Python](https://python.org/downloads)
- [Rust](https://rust-lang.org/tools/install)

<!-- Add project components and their documentations below -->

## Web

This directory contains the source code for our web application which also
includes our public-facing landing pages. The web application is built using
TypeScript, SvelteKit and TailwindCSS. To run this project locally for
development, you can use the following commands:

```bash
# Set up the web app for local development.
make setup_web

# Change the directory to the web app.
cd web

# Run the web app on localhost:5173.
npm run dev
```

### Style Guide

When writing code for the web application, we should follow the style guide
enforced by our ESLint and Prettier configurations. In addition to the GitHub
Actions we have set up to lint and format our code, we can also run the
following commands locally:

```bash
# Format the code using Prettier.
npm run format

# Lint the code using ESLint and check for formatting issues.
npm run lint
```

Please also follow the existing naming conventions and patterns:

- Kebab Case: Files, directories, and CSS classes
- Camel Case: TypeScript variables and functions
- Pascal Case: TypeScript classes and components
- Uppercase: Constants and environment variables

## Templates

This directory contains email templates that are used during authentication
workflows in Supabase. We use Jinja2 templating to generate these email
templates from a base template. If we want to modify the email templates, we can
do so by modifying the Jinja2 templates in the emails directory.

We can run the following commands to start working with the email templates:

```bash
# Set up the email templates directory for local development.
make setup_templates

# Change the directory to the email templates.
cd templates

# Generate the HTML email templates.
python build.py
```

This command will generate the HTML email templates in the build directory.
Unfortunately, since Supabase doesn't have an API to update email templates, we
have to manually copy the HTML email templates into the Supabase dashboard.

## Server

This directory contains the source code for our API server that coordinates the
document ingestion and retrieval workflows with the clients and extractor
workers. The server is built with Rust and contains two server implementations:
a REST API server, **Interface**, and a gRPC server, **Coordinator**.

- Interface Server (Axum): Interacts with the client and web application.
- Coordinator Server (Tonic): Interacts with the extractor workers.

To start working on the API server locally, you can use the following commands:

```bash
# Run the server setup script for local development.
make setup_server

# Change the directory to the server.
cd server

# Run the API server.
cargo run start
```

### Style Guide

When writing code for the API server, we should follow the standard Rust style
guide. If you are not familiar, you can learn more about the style guide here:
[Rust Style Guide](https://doc.rust-lang.org/style-guide). Additionally, we can
run the following command to check for formatting and linting issues:

```bash
# Check for linting issues using Clippy.
cargo clippy

# Format the code using Rustfmt.
cargo fmt
```

## Extractor

This directory contains the source code for our extractor workers which are
responsible to extract features from PDF documents. An extractor worker consists
of 2 processes running concurrently: **Extraction Loop** and **Embedding API**.

- Extraction Loop: Extracts features from the PDF documents during ingestion.
- Embedding API: Generates embeddings for the retrieval workflow.

To get started with developing the extractor, use the following commands:

```bash
# Install Poetry if you haven't already.
pip install poetry

# Run the extractor setup script.
make setup_extractor

# Change the directory to the extractor.
cd extractor

# Get instructions to run the extractor.
poetry run cli --help
```

### Style Guide

The extractor is written in Python and we should follow the PEP 8 style guide
when writing code for the extractor. If you are not familiar, you can learn more
about the style guide here: [PEP 8 Style Guide](https://pep8.org). Additionally,
please follow the existing naming conventions and patterns:

- Snake Case: Variables, functions, files, and modules
- Pascal Case: Classes
- Uppercase: Constants and environment variables

Note: Camel case and kebab case should be avoided in the extractor.

We use Black to format our Python code and we can run a command to format the
code automatically. Please make sure to run this command before committing your
changes. Also, please note that this command doesn't fix issues related with the
naming conventions.

```bash
poetry run black .
```
