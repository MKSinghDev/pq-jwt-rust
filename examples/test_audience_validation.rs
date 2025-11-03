use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Testing audience validation in verifier...\n");

    // Generate keypair
    let (private_key, public_key) = pq_jwt::generate_keypair(pq_jwt::MlDsaAlgo::Dsa65)
        .expect("Failed to generate keypair");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create JWT WITHOUT audience claim (using simple sign function)
    let (jwt, _) = pq_jwt::signer::sign(
        pq_jwt::MlDsaAlgo::Dsa65,
        "https://test.com",
        now + 3600,
        &private_key,
    )
    .expect("Failed to sign");

    println!("✓ JWT created WITHOUT audience claim");

    // Try to verify WITH audience requirement (issuer is also required)
    let verifier = pq_jwt::verifier::Builder::new()
        .public_key(&public_key)
        .issuer("https://test.com") // Required
        .audience("https://api.myapp.com") // Optional but we're setting it
        .build()
        .expect("Failed to build verifier");

    println!("✓ Verifier built WITH audience requirement\n");

    match verifier.verify(&jwt) {
        Ok(payload) => {
            println!("❌ BUG: Verification succeeded when it should have failed!");
            println!("   Payload: {}", payload);
            std::process::exit(1);
        }
        Err(e) => {
            println!("✓ CORRECT: Verification failed as expected");
            println!("   Error: {}", e);
        }
    }
}
