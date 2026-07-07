# KPHIS `kphis-sqlx-tester`

This library implements tests for `kphis-backend`'s SQLx database.

## HOW
- Start MariaDB docker
    ```bash
    docker run -d -p 3306:3306 bantenitsolutions/mariadb-lite
    ```
- Run `cargo test -- --test-threads=1`

---
This crate is part of the [KPHIS](https://github.com/Marisada/kphis) project.

> Inspired by [sqlx-db-tester](https://github.com/tyrchen/sqlx-db-tester)  