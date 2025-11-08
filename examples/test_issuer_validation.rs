use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Testing issuer validation scenarios...\n");

    let (private_key, public_key) =
        pq_jwt::generate_keypair(pq_jwt::MlDsaAlgo::Dsa65).expect("Failed to generate keypair");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create JWT with issuer="https://test.com"
    let (jwt, _, _) = pq_jwt::signer::sign(
        pq_jwt::MlDsaAlgo::Dsa65,
        "https://test.com",
        now + 3600,
        &private_key,
    )
    .expect("Failed to sign");

    println!("✓ JWT created with issuer='https://test.com'\n");

    // Scenario 1: Try to build verifier WITHOUT issuer (should fail)
    println!("Scenario 1: Try to build verifier WITHOUT issuer");
    let result1 = pq_jwt::verifier::Builder::new()
        .public_key(&public_key)
        .audience("https://api.myapp.com")
        .build();

    match result1 {
        Ok(_) => {
            println!("  ❌ Result: Build succeeded (should have failed!)\n");
        }
        Err(e) => {
            println!("  ✓ Result: Build failed as expected - {}\n", e);
        }
    }

    // Scenario 2: Verify WITH correct expected issuer
    println!("Scenario 2: Verifier WITH correct issuer");
    let verifier2 = pq_jwt::verifier::Builder::new()
        .public_key(&public_key)
        .issuer("https://test.com") // Correct issuer
        .build()
        .expect("Failed to build verifier");

    match verifier2.verify(&jwt) {
        Ok(_) => println!("  ✓ Result: Verification succeeded\n"),
        Err(e) => println!("  ❌ Result: Failed - {}\n", e),
    }

    // Scenario 3: Verify WITH wrong expected issuer
    println!("Scenario 3: Verifier WITH wrong issuer");
    let verifier3 = pq_jwt::verifier::Builder::new()
        .public_key(&public_key)
        .issuer("https://wrong.com") // Wrong issuer
        .build()
        .expect("Failed to build verifier");

    match verifier3.verify(&jwt) {
        Ok(_) => println!("  ❌ Result: Verification succeeded (should have failed!)\n"),
        Err(e) => println!("  ✓ Result: Failed - {}\n", e),
    }
}
