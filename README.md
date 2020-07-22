# Postgres Diesel
Example of simple usage of Postgres [Diesel](https://docs.rs/diesel/1.4.5/diesel/) and [r2d2](https://docs.rs/r2d2/0.8.9/r2d2/) with [Actix Web](https://actix.rs).

## Requirement
- Basic Rust tooling (cargo, etc.)
- Postgresql

## Connection Url
Diesel get database url from `.env` with key of `database_url`.

## What is schema.rs?
`schema.rs` is auto generated schema by diesel. To generate one:
1. run
```bash
diesel migration generate users
```

2. Navigate to `migrations` folder then edit `up.sql`:
```sql
-- Your SQL goes here
CREATE TABLE "users" (
    username VARCHAR PRIMARY KEY,
    password VARCHAR NOT NULL
)
```
and `down.sql`

```sql
-- This file should undo anything in `up.sql`
DROP TABLE "users"
```

3. Run in bash:
```bash
diesel migration run
```

Then `schema.rs` should be auto-generated

## How diesel access field?
Diesel access field by importing
```rust
use crate::schema::<schema name>::dsl::*;

// Suppose my schema is label as above in section `What is schema.rs?`
use crate::schema::users::dsl::*;
```

Now you can access field by:
```rust
users
    .filter(
        // Here from field `username`
        username.eq(self.username.to_owned())
    )
    .filter(
        // Here from field `username`
        password.eq(self.password.to_owned())
    )
    .first::<User>(connection)
    .optional()?;
```

## What's r2d2
[r2d2](https://docs.rs/r2d2/0.8.9/r2d2/) is a library to create [database connection pool](https://stackoverflow.com/questions/4041114/what-is-database-pooling).