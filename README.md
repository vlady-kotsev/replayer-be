# Replayer Backend

A Rust backend service for game encryption key management, integrated with the Solana blockchain for signature verification and NFT-based access control.

## Tech Stack

- **Axum** - HTTP framework
- **SQLx + PostgreSQL** - Database
- **Solana** - Signature verification & NFT validation (via `solana-keypair`, `solana-client`, `mpl-core`)
- **AES-256-GCM** - Encryption key generation
- **Tower** - Middleware layers

## Project Structure

```
src/
├── app/          # App initialization, router setup, shared state
├── client/       # Solana RPC client wrapper
├── config/       # Config loading (TOML)
├── db/           # Database connection pool
├── errors.rs     # Centralized error types
├── handler/      # Route handlers + request/response types
├── middleware/    # Tower middleware (signature recovery, NFT validation)
├── model/        # Domain models
├── repository/   # Database queries (DTOs + raw SQL)
├── routes/       # Router definitions
├── service/      # Business logic (game, key, signer)
└── utils/        # Custom Serde deserializers for Solana types
```

## Configuration

`config/config.toml`:

```toml
[app]
host = "0.0.0.0"
port = 3003
database_url = "postgres://user:pass@localhost:5432/replayer"

[solana]
keypair_bytes = "<base64-encoded 64-byte keypair>"
rpc_url = "https://api.devnet.solana.com"
program_id = "<base58 program address>"
```

## Database Setup

Requires PostgreSQL. A Docker Compose file is included for local development:

```bash
docker-compose up -d
```

Run migrations with SQLx:

```bash
sqlx migrate run
```

**Schema:**

```sql
CREATE TABLE games (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    developer VARCHAR(255) NOT NULL,
    encryption_key TEXT NOT NULL,
    nonce TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    CONSTRAINT unique_user_game UNIQUE (name, developer)
);
```

## API Endpoints

### `GET /games`

Returns all registered games.

**Response:**
```json
[{ "name": "my-game", "developer": "<base58 address>" }]
```

### `POST /games`

Creates a new game and returns the generated encryption key.

**Middleware:** Signature verification

**Request:**
```json
{
  "signature": "<base58 signature>",
  "name": "my-game",
  "valid_period": 1234567890,
  "developer": "<base58 address>",
  "player": "<base58 address>"
}
```

> For game creation, `player` and `developer` are the same address.

**Response:**
```json
{
  "encryption_key": "<base64 AES-256 key>",
  "nonce": "<base64 nonce>"
}
```

### `POST /keys`

Retrieves the encryption key for a game. Requires NFT ownership.

**Middleware:** Signature verification + NFT validation

**Request:**
```json
{
  "signature": "<base58 signature>",
  "name": "my-game",
  "valid_period": 1234567890,
  "developer": "<base58 address>",
  "player": "<base58 address>"
}
```

**Response:**
```json
{
  "encryption_key": "<base64 AES-256 key>",
  "nonce": "<base64 nonce>",
  "signature": "<base58 server signature>",
  "valid_period": 1234567890
}
```

## Middleware

### Signature Recovery (`RecoverSignatureLayer`)

Verifies that the request was signed by the `player`. The signed message format is:

```
Replayer: Register game '{name}' (valid until: {valid_period})
```

### NFT Validation (`ValidateNftLayer`)

Applied to the `/keys` route. Derives a PDA from `["game_key", developer, game_name, player]`, fetches the on-chain account, and verifies the player owns the NFT.

## Running

```bash
cargo run
```

The server starts on the host and port defined in `config.toml`.
