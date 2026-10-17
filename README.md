# Scholar's Mate

```sh
devenv up -d
devenv shell
psql "$DATABASE_URL" -f seeds/dev.sql
cargo dev
```

Open http://127.0.0.1:3000.
