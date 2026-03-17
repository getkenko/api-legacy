# ⚠️ Legal Notice / Licensing Warning

This repository is a **re-upload of the original Kenko API source code**.

The project was previously published without authorization and under an incorrect license.

That licensing was **invalid and does not reflect the actual terms** under which this code was created and distributed.

## Correct License

This project is licensed under the **Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)** license.

All prior distributions under MIT should be considered **incorrect and legally void**.

## Important Notes

- This repository represents the **official and authorized version** of the project.
- The licensing has been **corrected to reflect the original intent and legal status**.
- If you have obtained this code from another source under different terms, please refer to this repository as the **canonical source of truth**.

For full license terms, see the [LICENSE](LICENSE) file.

---

# Kenko API

This is an official repository containing source code of the Kenko mobile app's API.

It's written in Rust using Axum web framework, SQLx as database driver, PostgreSQL for database, and Redis for caching.

## Documentation

This project was never intended to be made public therefore it lacks any code documentation and comments.

However you can find documentation of the API routes and usage on [Postman](https://documenter.getpostman.com/view/38099323/2sBXigLsp3).

## Building

To build the API you're expected to have some basic knowledge of how Rust, Axum, Postgres, and REST APIs work.

1. Generate asymmetric key pair for JWT signing/verification

```bash
./keys/generate.sh
```

2. Copy `.env.example` to `.env` and change the `DATABASE_URL` to your Postgres credentials

3. Point `redis_url` in `config.toml` to your running Redis instance

4. Execute all migrations (in order) from `./migrations/` directory

5. Run the development server

```bash
cargo run
```

6. Build the API for production

```bash
cargo build --release
```

## License

This project is licensed under Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0).

You can read more in [LICENSE](LICENSE) file.
