# 🔐 pq-jwt

[![Crates.io](https://img.shields.io/crates/v/pq-jwt.svg)](https://crates.io/crates/pq-jwt)
[![Documentation](https://docs.rs/pq-jwt/badge.svg)](https://docs.rs/pq-jwt)
[![CircleCI](https://circleci.com/gh/MKSinghDev/pq-jwt-rust/tree/trunk.svg?style=shield)](https://app.circleci.com/pipelines/github/MKSinghDev/pq-jwt-rust?branch=trunk)
![CI](https://github.com/MKSinghDev/pq-jwt-rust/workflows/CI/badge.svg)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Post-Quantum JWT** - A quantum-resistant JWT implementation using ML-DSA (Module-Lattice Digital Signature Algorithm) signatures.

> 🛡️ **Future-proof your authentication** - Protect your JWTs against quantum computer attacks with NIST-standardized post-quantum cryptography.

## 🌟 Features

- ✅ **Quantum-Resistant** - Uses ML-DSA (FIPS 204) signatures that remain secure even against quantum attacks
- ✅ **Multiple Security Levels** - Choose from ML-DSA-44, ML-DSA-65, or ML-DSA-87 based on your needs
- ✅ **Standards Compliant** - JWT format following RFC 7519
- ✅ **Flexible API** - Simple functions and advanced Builder patterns
- ✅ **Key Management** - Built-in support for saving keys to files
- ✅ **Key Rotation** - Support for `kid` (Key ID) in JWT headers
- ✅ **Zero Dependencies Bloat** - Minimal, focused dependencies
- ✅ **Easy to Use** - Simple, intuitive API
- ✅ **Well Tested** - Comprehensive test coverage with unit and integration tests
- ✅ **Pure Rust** - Memory-safe implementation with no unsafe code

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pq-jwt = "0.1.0"
```

## 🚀 Quick Start

```rust
use pq_jwt::{generate_keypair, sign, verify, MlDsaAlgo};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), String> {
    // 1. Generate a keypair
    let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65)?;

    // 2. Create and sign a JWT with issuer and expiration
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let (jwt, _) = sign(
        MlDsaAlgo::Dsa65,
        "https://myapp.com",      // Issuer
        now + 3600,                // Expires in 1 hour
        &private_key
    )?;

    println!("JWT: {}", jwt);

    // 3. Verify the JWT
    let verified_payload = verify(&jwt, &public_key)?;
    println!("Verified payload: {}", verified_payload);

    println!("✓ JWT verified successfully!");
    Ok(())
}
```

## 📚 Usage Examples

### Basic Authentication Token (Simple API)

```rust
use pq_jwt::{generate_keypair, sign, verify, MlDsaAlgo};
use std::time::{SystemTime, UNIX_EPOCH};

// Generate long-term keypair (store securely!)
let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65)?;

// Create user session token
let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
let (jwt, _) = sign(
    MlDsaAlgo::Dsa65,
    "https://myapp.com",    // Issuer
    now + 3600,              // Expires in 1 hour
    &private_key
)?;

// Later: verify the token
let payload = verify(&jwt, &public_key)?;
println!("Authenticated user: {}", payload);
```

### Advanced Authentication Token (Builder API with Custom Claims)

```rust
use pq_jwt::signer::Builder;
use pq_jwt::MlDsaAlgo;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;

let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65)?;
let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Create signer with all standard claims and custom data
let signer = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&private_key)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .subject("user123")
    .audience("https://api.myapp.com")
    .custom_claims(json!({
        "name": "Alice",
        "role": "admin",
        "permissions": ["read", "write", "delete"]
    }))
    .build()?;

let (jwt, _) = signer.sign()?;

// Verify
let payload = verify(&jwt, &public_key)?;
println!("Token payload: {}", payload);
```

### Generate and Save Keys to File

```rust
use pq_jwt::keygen::Builder;
use pq_jwt::MlDsaAlgo;

// Generate and save to default location (keys/)
let (private_key, public_key) = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .save_to_file()
    .generate()?;

// Or save to custom location
let (private_key, public_key) = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .save_to_file_at("./my-secure-keys")
    .generate()?;

// Files created:
// - ml_dsa_65_1704139200_private.key
// - ml_dsa_65_1704139200_public.key (derived from private key)
```

### Load Keys from File

```rust
use pq_jwt::keygen::{Builder, KeySource};
use pq_jwt::MlDsaAlgo;

// Load from default location (keys/) - picks latest by timestamp
let (private_key, public_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    .file()?;

// Load from custom location
let (private_key, public_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    .file_at("./my-secure-keys")?;

// Public key is automatically derived from private key
assert_eq!(source, KeySource::Loaded);
```

### Load or Generate Keys (Automatic Fallback)

```rust
use pq_jwt::keygen::{Builder, KeySource};
use pq_jwt::MlDsaAlgo;

// Try to load existing key, generate if missing
let (private_key, public_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    .file()?;

match source {
    KeySource::Loaded => println!("Using existing key"),
    KeySource::Generated => println!("Generated new key and saved"),
}

// Custom location
let (private_key, public_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    .file_at("./my-secure-keys")?;

// Perfect for server initialization - always has a valid key!
```

### Load Keys from String (Database/Environment)

```rust
use pq_jwt::keygen::{Builder, KeySource};
use pq_jwt::MlDsaAlgo;

// Load private key from database or environment
let private_key_from_db = std::env::var("JWT_PRIVATE_KEY")?;

// Derive public key from private key
let (private_key, public_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    .private_key_str(&private_key_from_db)?;

assert_eq!(source, KeySource::Loaded);
// Use the keys for signing/verification
```

### Key Rotation with Key ID (kid)

The Key ID (kid) is automatically generated from the public key using SHA-256, ensuring consistent identification across key rotations.

```rust
use pq_jwt::signer::Builder as SignerBuilder;
use pq_jwt::verifier::Builder as VerifierBuilder;
use pq_jwt::{generate_keypair, MlDsaAlgo};
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Generate keypair
let (priv_key_v2, pub_key_v2) = generate_keypair(MlDsaAlgo::Dsa65)?;

// Create signer (kid is auto-generated from public key)
let signer = SignerBuilder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&priv_key_v2)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .build()?;

let (jwt, _) = signer.sign()?;

// Verify (kid from JWT header can be used to identify which key to use)
let verifier = VerifierBuilder::new()
    .public_key(&pub_key_v2)
    .build()?;

let payload = verifier.verify(&jwt)?;
```

### Reusable Signer and Verifier

```rust
use pq_jwt::signer::Builder as SignerBuilder;
use pq_jwt::verifier::Builder as VerifierBuilder;
use pq_jwt::MlDsaAlgo;
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Create once, use many times
let signer = SignerBuilder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&private_key)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .build()?;

// Sign (no parameters needed - uses configured claims)
let (jwt1, _) = signer.sign()?;
let (jwt2, _) = signer.sign()?;
let (jwt3, _) = signer.sign()?;

// Create reusable verifier
let verifier = VerifierBuilder::new()
    .public_key(&public_key)
    .build()?;

// Verify multiple tokens
for jwt in [jwt1, jwt2, jwt3] {
    match verifier.verify(&jwt) {
        Ok(payload) => println!("Valid: {}", payload),
        Err(e) => println!("Invalid: {}", e),
    }
}
```

### API Authentication

```rust
use pq_jwt::{generate_keypair, MlDsaAlgo};
use pq_jwt::signer::Builder;
use pq_jwt::verifier;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Server initialization
let (server_private_key, server_public_key) =
    generate_keypair(MlDsaAlgo::Dsa65)?;

// Issue API token with custom claims
let signer = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&server_private_key)
    .issuer("https://api.myapp.com")
    .expiration(now + 86400)  // 24 hours
    .subject("ak_live_123456")
    .custom_claims(json!({
        "scope": ["read", "write"],
        "rate_limit": 1000
    }))
    .build()?;

let (api_token, _) = signer.sign()?;

// Client sends: Authorization: Bearer <api_token>
// Server verifies:
match verifier::verify(&api_token, &server_public_key) {
    Ok(claims) => println!("Valid API token: {}", claims),
    Err(e) => println!("Invalid token: {}", e),
}
```

### Custom Payload with Type Safety

```rust
use pq_jwt::signer::Builder;
use pq_jwt::{verify, MlDsaAlgo};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct CustomData {
    user_id: u64,
    role: String,
    permissions: Vec<String>,
}

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

let custom_data = CustomData {
    user_id: 42,
    role: "admin".to_string(),
    permissions: vec!["read".to_string(), "write".to_string()],
};

// Build JWT with standard claims + custom data
let signer = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&private_key)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .subject("user_42")
    .custom_claims(serde_json::to_value(&custom_data)?)
    .build()?;

let (jwt, _) = signer.sign()?;

// Later... verify and extract
let verified = verify(&jwt, &public_key)?;
let payload: serde_json::Value = serde_json::from_str(&verified)?;
let custom: CustomData = serde_json::from_value(payload)?;
println!("User {} has role: {}", custom.user_id, custom.role);
```

## 🔑 Security Levels

Choose the right security level for your use case:

| Variant | NIST Level | Signature Size | Key Gen | Sign | Verify | Use Case |
|---------|-----------|----------------|---------|------|--------|----------|
| **ML-DSA-44** | Category 2 | ~2.4 KB | ~200 µs | ~460 µs | ~140 µs | IoT devices, low-power systems |
| **ML-DSA-65** | Category 3 | ~3.3 KB | ~350 µs | ~930 µs | ~220 µs | **Recommended for most applications** |
| **ML-DSA-87** | Category 5 | ~4.6 KB | ~440 µs | ~550 µs | ~315 µs | High-security requirements, long-term secrets |

### Security Level Comparison

- **NIST Category 2** ≈ AES-128 security
- **NIST Category 3** ≈ AES-192 security (Recommended)
- **NIST Category 5** ≈ AES-256 security

### Choosing an Algorithm

```rust
use pq_jwt::MlDsaAlgo;

// For most web applications (recommended)
let algo = MlDsaAlgo::Dsa65;

// For IoT or bandwidth-constrained environments
let algo = MlDsaAlgo::Dsa44;

// For maximum security (government, financial)
let algo = MlDsaAlgo::Dsa87;
```

## 🎯 Performance

Benchmarked on Apple M1 Pro (release build):

```
ML-DSA-65 Performance:
├─ Key Generation: ~350 µs (2,857 ops/sec)
├─ Signing:        ~930 µs (1,075 ops/sec)
└─ Verification:   ~220 µs (4,545 ops/sec)

Token Size: ~4.5 KB (vs ~300 bytes for ECDSA)
```

### Performance Tips

1. **Cache Keys**: Generate keypairs once and reuse them
2. **Pre-verify Format**: Check JWT structure before cryptographic verification
3. **Use ML-DSA-44**: If bandwidth is critical and security level 2 is acceptable
4. **Batch Operations**: Verify multiple tokens in parallel for better throughput

## 📊 Size Comparison

| Algorithm | Private Key | Public Key | Signature | Total JWT |
|-----------|------------|------------|-----------|-----------|
| **ECDSA P-256** | 32 bytes | 64 bytes | 64 bytes | ~300 bytes |
| **RSA-2048** | 1.2 KB | 270 bytes | 256 bytes | ~800 bytes |
| **ML-DSA-44** | 2.5 KB | 1.3 KB | 2.4 KB | ~3.3 KB |
| **ML-DSA-65** | 4 KB | 1.9 KB | 3.3 KB | ~4.5 KB |
| **ML-DSA-87** | 4.9 KB | 2.6 KB | 4.6 KB | ~6.2 KB |

> ⚠️ **Trade-off**: Post-quantum signatures are larger, but provide quantum resistance. The size increase is the price of security against quantum attacks.

## 🛠️ API Reference

### Simple API (Convenience Functions)

#### `generate_keypair(algo: MlDsaAlgo) -> Result<(String, String), String>`

Generates a new keypair for the specified algorithm.

**Returns**: `(private_key_hex, public_key_hex)`

```rust
let (private_key, public_key) = generate_keypair(MlDsaAlgo::Dsa65)?;
```

#### `sign(algo: MlDsaAlgo, iss: &str, exp: u64, private_key_hex: &str) -> Result<(String, String), String>`

Signs JWT claims and returns a JWT with the public key.

**Parameters**:
- `algo` - ML-DSA algorithm variant
- `iss` - Issuer (REQUIRED)
- `exp` - Expiration time as Unix timestamp in seconds (REQUIRED)
- `private_key_hex` - Hex-encoded private key

**Returns**: `(jwt, public_key_hex)`

**Note**: The `iat` (issued at) claim defaults to the current time.

```rust
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
let (jwt, pub_key) = sign(
    MlDsaAlgo::Dsa65,
    "https://myapp.com",
    now + 3600,
    &private_key
)?;
```

#### `verify(jwt: &str, public_key_hex: &str) -> Result<String, String>`

Verifies a JWT and returns the decoded payload.

**Returns**: `payload` if valid, error otherwise

```rust
let payload = verify(&jwt, &public_key)?;
```

### Builder API (Advanced)

#### `keygen::Builder`

**Generation Methods:**
- `Builder::new()` - Create builder for generation
- `.algorithm(MlDsaAlgo)` - Set the algorithm variant
- `.save_to_file()` - Save keys to default location (`keys/`)
- `.save_to_file_at(path)` - Save keys to custom path
- `.generate()` - Generate keypair (and save if configured)
- Returns: `(private_key_hex, public_key_hex)`

**Loading Methods:**
- `Builder::from(algo)` - Create builder for loading (error if missing)
- `Builder::load_or_generate(algo)` - Load or auto-generate if missing
- `.file()` - Load from default location (`keys/`), picks latest by timestamp
- `.file_at(path)` - Load from custom path, picks latest by timestamp
- `.private_key_str(hex)` - Load from hex string, derives public key
- Returns: `(private_key_hex, public_key_hex, KeySource)`

```rust
use pq_jwt::keygen::{Builder, KeySource};

// Generate and save
let (priv_key, pub_key) = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .save_to_file_at("./secure-keys")
    .generate()?;

// Load from file (error if missing)
let (priv_key, pub_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    .file_at("./secure-keys")?;

// Load or generate (auto-fallback)
let (priv_key, pub_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    .file_at("./secure-keys")?;

// Load from string
let (priv_key, pub_key, source) = Builder::from(MlDsaAlgo::Dsa65)
    .private_key_str(&hex_string)?;
```

#### `signer::Builder`

**Configuration Methods:**
- `.algorithm(MlDsaAlgo)` - Set the algorithm variant (REQUIRED)
- `.private_key(&str)` - Set the private key (REQUIRED)

**Standard JWT Claims Methods:**
- `.issuer(&str)` - Set `iss` claim (REQUIRED)
- `.expiration(u64)` - Set `exp` claim as Unix timestamp (REQUIRED)
- `.subject(&str)` - Set `sub` claim (optional)
- `.audience(&str)` - Set `aud` claim (optional)
- `.issued_at(Option<u64>)` - Set `iat` claim, defaults to signing time if not set (optional)
- `.not_before(u64)` - Set `nbf` claim as Unix timestamp (optional)
- `.jwt_id(&str)` - Set `jti` claim (optional)
- `.custom_claims(serde_json::Value)` - Add custom claims (optional)

**Build Method:**
- `.build()` - Build Signer instance, returns `Result<Signer, String>`

**Signer Methods:**
- `.sign()` - Sign the configured claims, returns `Result<(String, String), String>`

**Notes:**
- The Key ID (kid) is automatically generated from the public key using SHA-256
- The `iat` (issued at) defaults to the current signing time if not explicitly set
- Claims are validated before signing (`exp > iat`, `nbf <= iat`)
- Custom claims that duplicate standard claim keys are ignored

```rust
use pq_jwt::signer::Builder;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

let signer = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&priv_key)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .subject("user@example.com")
    .custom_claims(json!({
        "role": "admin",
        "permissions": ["read", "write"]
    }))
    .build()?;

let (jwt, pub_key) = signer.sign()?;
```

#### `verifier::Builder`

**Configuration Methods:**
- `.public_key(&str)` - Set the public key (REQUIRED)

**Claim Validation Methods (Optional):**
- `.issuer(&str)` - Set expected issuer for validation
- `.audience(&str)` - Set expected audience for validation
- `.subject(&str)` - Set expected subject for validation
- `.leeway(u64)` - Set time leeway in seconds for clock skew (default: 0)

**Build Method:**
- `.build()` - Build Verifier instance, returns `Result<Verifier, String>`

**Verifier Methods:**
- `.verify(&str)` - Verify JWT and return payload, returns `Result<String, String>`

**Automatic Validations (Always Performed):**
- ✅ Signature verification (cryptographic)
- ✅ Expiration check (`exp` must be in the future)
- ✅ Issuer existence (`iss` claim must exist and not be empty)

**Optional Validations (Configured via Builder):**
- Expected issuer matching
- Expected audience matching
- Expected subject matching
- Not before time (`nbf` if present in token)

```rust
use pq_jwt::verifier::Builder;

// Basic verification (auto-validates exp and iss)
let verifier = Builder::new()
    .public_key(&pub_key)
    .build()?;

let payload = verifier.verify(&jwt)?;

// Advanced verification with claim validation
let verifier = Builder::new()
    .public_key(&pub_key)
    .issuer("https://myapp.com")        // Validate issuer matches
    .audience("https://api.myapp.com")  // Validate audience matches
    .subject("user@example.com")        // Validate subject matches
    .leeway(60)                         // Allow 60s clock skew
    .build()?;

let payload = verifier.verify(&jwt)?;
```

### Enums

#### `MlDsaAlgo`

Available algorithm variants:

- `MlDsaAlgo::Dsa44` - NIST Category 2
- `MlDsaAlgo::Dsa65` - NIST Category 3 (Recommended)
- `MlDsaAlgo::Dsa87` - NIST Category 5

**Traits:** `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`

#### `KeySource`

Indicates the source of a keypair when using `load_or_generate`:

- `KeySource::Loaded` - Successfully loaded existing key from file or string
- `KeySource::Generated` - Generated new key (file was missing or corrupt)

**Traits:** `Debug`, `Clone`, `PartialEq`, `Eq`

```rust
use pq_jwt::keygen::{Builder, KeySource};

let (priv_key, pub_key, source) = Builder::load_or_generate(MlDsaAlgo::Dsa65)
    .file()?;

match source {
    KeySource::Loaded => println!("Reusing existing key"),
    KeySource::Generated => println!("Created new key"),
}
```

## 🔄 Migration Guide

### From v0.1.x to v0.2.x

**Breaking Change**: The `sign()` function signature has changed to require `iss` and `exp` parameters.

**Old API (v0.1.x)**:
```rust
let payload = r#"{"sub": "user123", "exp": 1735689600}"#;
let (jwt, _) = sign(MlDsaAlgo::Dsa65, payload, &priv_key)?;
```

**New API (v0.2.x)**:
```rust
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
let (jwt, _) = sign(
    MlDsaAlgo::Dsa65,
    "https://myapp.com",  // issuer (required)
    now + 3600,            // expiration (required)
    &priv_key
)?;
```

**For more complex claims, use the Builder API**:
```rust
use pq_jwt::signer::Builder;
use serde_json::json;

let signer = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&priv_key)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .subject("user123")
    .custom_claims(json!({
        "role": "admin",
        "permissions": ["read", "write"]
    }))
    .build()?;

let (jwt, _) = signer.sign()?;
```

### New Features Available

**Key File Management:**
```rust
// Old way - manual file handling
let (priv_key, pub_key) = generate_keypair(MlDsaAlgo::Dsa65)?;
std::fs::write("private.key", &priv_key)?;
std::fs::write("public.key", &pub_key)?;

// New way - built-in
use pq_jwt::keygen::Builder;
let (priv_key, pub_key) = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .save_to_file()
    .generate()?;
```

**Key Rotation:**
```rust
// New: kid is automatically generated for key rotation
use pq_jwt::signer::Builder;
let signer = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&priv_key)
    .build()?;
// The kid in the JWT header can be used to identify which public key to use
```

**Reusable Instances:**
```rust
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// New: Create once, use multiple times
let signer = signer::Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&priv_key)
    .issuer("https://myapp.com")
    .expiration(now + 3600)
    .build()?;

// Sign (no parameters needed - uses configured claims)
let (jwt1, _) = signer.sign()?;
let (jwt2, _) = signer.sign()?;
```

**JWT Claims Validation:**
```rust
// New: Automatic validation of JWT claims
// - exp > iat (expiration must be after issued at)
// - nbf <= iat (not before must be before or equal to issued at)
// Validation happens automatically when calling sign()
```

## 🔒 Security Considerations

### Key Management

- **Never commit private keys** to version control
- **Rotate keys regularly** (every 90 days recommended)
- **Use environment variables** or secret management systems
- **Store keys encrypted** at rest
- **Use file storage with proper permissions** (0600 for private keys)

```rust
// ✓ Good - Environment variables
let private_key = std::env::var("JWT_PRIVATE_KEY")?;

// ✓ Good - Secure file storage
use pq_jwt::keygen::Builder;
let (priv_key, pub_key) = Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .save_to_file_at("/secure/keys")
    .generate()?;

// ✗ Bad - Hardcoded
let private_key = "4343e9e24838dbd8..."; // Never do this
```

### Key Rotation Strategy

```rust
// Step 1: Generate new keypair (kid will be auto-generated)
let (new_priv, new_pub) = keygen::Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .save_to_file_at("/keys/v3")
    .generate()?;

// Step 2: Create new signer (kid is auto-generated from public key)
let signer = signer::Builder::new()
    .algorithm(MlDsaAlgo::Dsa65)
    .private_key(&new_priv)
    .build()?;

// Step 3: Store the public key with its auto-generated kid for verification
// You can extract the kid from a signed JWT's header to identify which key to use
// Step 4: Keep old public keys for verification during transition period
// Step 5: Gradually phase out old keys
```

### Token Best Practices

1. **Always include expiration** (`exp` claim)
2. **Use short lifetimes** for sensitive operations (15 min - 1 hour)
3. **Implement token revocation** if needed
4. **Validate claims** after verification
5. **Use HTTPS** for token transmission

### Example with Expiration

```rust
use std::time::{SystemTime, UNIX_EPOCH};

let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
let exp = now + 3600; // 1 hour from now

let payload = format!(r#"{{
    "sub": "user123",
    "iat": {},
    "exp": {}
}}"#, now, exp);

let (jwt, _) = sign(MlDsaAlgo::Dsa65, &payload, &private_key)?;
```

## 🤔 Why Post-Quantum?

### The Quantum Threat

Quantum computers, when fully developed, will break current cryptographic systems:

- **RSA** - Vulnerable to Shor's algorithm
- **ECDSA** - Vulnerable to Shor's algorithm
- **Diffie-Hellman** - Vulnerable to quantum attacks

### Timeline

- **2023**: NIST standardizes post-quantum algorithms (ML-DSA = FIPS 204)
- **2025-2030**: Quantum computers may break RSA-2048
- **2030+**: All systems must use post-quantum crypto

### "Harvest Now, Decrypt Later"

Attackers can:
1. Intercept and store encrypted data today
2. Wait for quantum computers to become available
3. Decrypt the data retroactively

**Solution**: Start using post-quantum crypto NOW to protect long-term secrets.

## 🆚 Comparison with Classical JWT

| Feature | pq-jwt (ML-DSA) | Classical (ECDSA) |
|---------|----------------|-------------------|
| **Quantum Resistant** | ✅ Yes | ❌ No |
| **NIST Standardized** | ✅ FIPS 204 | ✅ FIPS 186 |
| **Token Size** | 3-6 KB | ~300 bytes |
| **Sign Speed** | ~0.5-1 ms | ~0.05-0.1 ms |
| **Verify Speed** | ~0.2-0.3 ms | ~0.1-0.2 ms |
| **Security Level** | 128-256 bit | 128-256 bit |
| **Future Proof** | ✅ Yes | ❌ Vulnerable to quantum |

## 🔧 Integration Examples

### With Actix Web

```rust
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use pq_jwt::{verify, MlDsaAlgo};

async fn protected_route(req: HttpRequest) -> Result<String> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing token"))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid format"))?;

    let public_key = std::env::var("JWT_PUBLIC_KEY")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Config error"))?;

    match verify(token, &public_key) {
        Ok(payload) => Ok(format!("Authenticated: {}", payload)),
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}
```

### With Axum

```rust
use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};
use pq_jwt::verify;

async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let public_key = std::env::var("JWT_PUBLIC_KEY")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    verify(token, &public_key)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_full_workflow

# Run benchmarks
cargo test --release
```

## 📖 Further Reading

- [NIST FIPS 204 - ML-DSA Standard](https://csrc.nist.gov/pubs/fips/204/final)
- [Post-Quantum Cryptography FAQ](https://csrc.nist.gov/projects/post-quantum-cryptography/faqs)
- [JWT RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519)
- [NIST Post-Quantum Standards](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
git clone https://github.com/MKSinghDev/pq-jwt-rust.git
cd pq-jwt-rust
cargo build
cargo test
```

## 📄 License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## 👨‍💻 Author

**MKSingh** ([@MKSingh_Dev](https://x.com/MKSingh_Dev))

## ⭐ Star History

If you find this project useful, please consider giving it a star! ⭐

---

**Made with ❤️ for a quantum-safe future**
