# MES Gateway

A small Rust/Axum service that accepts semi-structured MES (Manufacturing Execution System) payloads and maps them to a consistent production telemetry format.

## Run

```sh
cargo run
```

The server listens on `http://localhost:8080`.

## API

- `GET /health/live` and `GET /health/ready` return `200 OK`.
- `POST /api/v1/ingest` accepts JSON and returns `200 Created` when mapping succeeds, or `400 Bad Request` for invalid or incomplete data.

Example:

```sh
curl -i http://localhost:8080/api/v1/ingest \
	-H 'content-type: application/json' \
	-d '{"station_id":"ASSEMBLY-02","timestamp":"1710001234","part_code":"BOLT-M8","status":"PASS","metrics":{"cycle_time":320}}'
```

Accepted statuses include `OK`, `SUCCESS`, `PASS`, `FAIL`, `ERROR`, and `ERR`. `timestamp` must be an integer string; `part_code` is required for a successful mapping. Ingestion currently transforms the payload but does not store or forward it.

Run tests with `cargo test`.
