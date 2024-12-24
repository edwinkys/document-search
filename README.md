![DocuLens](https://doculens-assets.s3.amazonaws.com/banners/logo.png)

DocuLens is a search API platform that allows users to build a retrieval system
for their own or their users' PDF documents. Some common use cases of DocuLens
are search engines for academic research papers or internal knowledge base
chatbots. At a high level, we have two main workflows in our system:
**Ingestion** and **Retrieval**.

This is a monorepo that contains the source code for our internal components.

These are some things you might need to have installed locally:

- [Node.js](https://nodejs.org/en/download/package-manager)
- [Python](https://python.org/downloads)
- [Rust](https://rust-lang.org/tools/install)

Additionally, you might want to set up a Python virtual environment:

```bash
# Create a virtual environment.
python -m venv .venv

# Activate the virtual environment.
source .venv/bin/activate

# Install the Python dependencies.
pip install -r requirements.txt
```

<!-- Add project components and their documentations below -->

## Web Application

This directory contains the source code for our web application which also
includes our public-facing landing pages. The web application is built using
TypeScript, SvelteKit and TailwindCSS. To run this project locally for
development, you can use the following commands:

```bash
# Install the project dependencies.
npm install

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

## Email Templates

This directory contains email templates that are used during authentication
workflows in Supabase. We use Jinja2 templating to generate the email templates.
If we want to modify the email templates, we can do so by modifying the Jinja2
templates in the emails directory.

After modifying the Jinja2 templates, run the following command to compile them
into HTML.

```bash
python build.py
```

This command will generate the HTML email templates in the build directory.
Unfortunately, since Supabase doesn't have an API to update email templates, we
have to manually copy the HTML email templates into the Supabase dashboard.

## API Server

This directory contains the source code for our API server that coordinates the
document ingestion and retrieval workflows with the extractor workers. The API
server is built with Rust and contains two server implementations: a REST API
server and a gRPC server.

- REST API Server (Axum): Interacts with the client and web application.
- gRPC Server (Tonic): Interacts with the extractor workers.

To run the API server locally, you can use the following commands:

```bash
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
