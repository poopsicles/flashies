# small backend for flashies

small proof of concept for the whole "lets reuse files already in the db" idea.

written in rust because i needed something quick and dirty, but guaranteed to work.

currently relies on just the hash of the file, maybe future versions would be more robust and check filename, size, date-modified.

## endpoints:

- `GET /`, returns the main page for testing.
- `GET /health`, returns `200`.
- `GET /check?sha=XXX`, returns `200 "in db"` if a file exists in the db with that hash, `404 "not in db"` otherwise
- `GET /get?sha=XXX`, returns a `200 <file with the given hash>` on success, `404` on error.
- `GET /all`, returns a `200 {<number of files in db> [list of files and hashes]}`
- `POST /post` returns `201` on success, `400` on error. the body is:

  ```text
  name: String,
  hash: String,
  data: Bytes,
  ```

so ideal flow would be:

1. Get a file from user, store in-memory.
2. Hash it clientside, and hit the `/check` endpoint to see if it exists.
3. If it does, yay.
4. If not, `/post` it.
5. List all the file names with `/all`, with little "Download" buttons by the side.
6. If one is clicked `/get` the file.

## running:

run with `docker compose up` if you prefer a containerised approach (this makes a Mysql container, waits for it to be initialised, builds the app as a container too, and exposes that)

clone and run with `cargo build` if you have the [rust toolchain](https://rustup.rs) installed (make sure to modify [.env](.env) to point to your own database).

## vid:

https://github.com/poopsicles/flashies/assets/87488715/f5a77cc9-5913-4375-b19c-4b8a86f60772



