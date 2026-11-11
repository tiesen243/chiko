# Chiko

Chiko is a Rust-based Cloudflare Worker API backed by Cloudflare D1. It provides user and post management endpoints, OpenAPI documentation, and a health check.

## Features

- Rust serverless API running on Cloudflare Workers
- Cloudflare D1 database integration
- User and post management endpoints
- OpenAPI specification and Scalar API reference

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)
- Cloudflare account with Workers and D1 access

## Development

Install the JavaScript tooling dependencies:

```sh
bun install
```

Start the local Worker development server:

```sh
bunx wrangler dev
```

The API exposes:

- `GET /` - Welcome response
- `GET /health` - Health check
- `GET /openapi.json` - OpenAPI specification
- `GET /docs` - Interactive Scalar API reference
- User and post routes registered by the application modules

## Deployment

Deploy the Worker with:

```sh
bunx wrangler deploy
```

The Worker and D1 binding are configured in `cloudflare.config.ts` and `wrangler.config.ts`.

## License

Chiko is available under the [MIT License](./LICENSE).