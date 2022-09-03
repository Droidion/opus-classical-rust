# Opus Classical

Written in Rust.

## Configuration

Have config file `config.yaml` in the root folder:

```yaml
application:
  port: 8000
database:
  host: "127.0.0.1"
  port: 5432
  username: "username"
  password: "password"
  database_name: "database"
  require_ssl: false
redis_uri: "redis://127.0.0.1:6379"
```