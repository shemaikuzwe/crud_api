# crud_api

Simple CRUD API in Rust + Diesel + Postgres + JWT auth. Same API implemented with three different HTTP frameworks (one per branch).

## Branches

| Branch | Framework | Description |
|---|---|---|
| `main` | [yam-http](https://github.com/shemaikuzwe/yam) | Default. Custom lightweight framework (`yam`). |
| `with_axum` | [axum 0.8](https://github.com/tokio-rs/axum) | Same controllers/services ported to Axum + tower-http. |
| `with_actix_web` | [actix-web 4](https://github.com/actix/actix-web) | Same controllers/services ported to Actix Web. |

Switch branches to compare routing, middleware, and handler styles with identical business logic:

```bash
git checkout main            # yam
git checkout with_axum       # axum
git checkout with_actix_web  # actix-web
```

## Stack

- **Runtime:** Tokio
- **DB:** Postgres + Diesel ORM
- **Auth:** bcrypt + JWT (jsonwebtoken with `rust_crypto`)
- **Logging:** tracing / tracing-subscriber

## Requirements

- Rust (edition 2024)
- Postgres
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Setup

```bash
git clone https://github.com/shemaikuzwe/crud_api.git
cd crud_api

# env
cp .env.example .env  
```

```bash
# run migrations
diesel migration run

# run (branch-specific)
cargo run
# -> http://localhost:3000
```

## Project Structure

```
src/
  main.rs        # router + server bootstrap
  lib.rs         # env / db connection
  admin/         # admin CRUD (controller, service, dtos)
  auth/          # signup / login
  middleware/    # auth + logging
  models.rs
  schema.rs
migrations/
```

## License

MIT
