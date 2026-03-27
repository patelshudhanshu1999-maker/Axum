use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(hashed_password) {
        Ok(parsed_hash) => argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok(),
        Err(_) => false,
    }
}





































// use std::time::Instant;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_hash_and_verify() {
//         let password = "my_secret_password";
//         let hashed = hash_password(password);
//         assert!(verify_password(password, &hashed));
//         assert!(!verify_password("wrong_password", &hashed));
//     }

//     #[test]
//     fn benchmark_password_hashing() {
//         println!("=== Password Hashing Performance Benchmark ===");
        
//         // Test data
//         let passwords = vec![
//             "password123",
//             "my_secure_password",
//             "another_password",
//             "test_password_123",
//             "complex_password!@#",
//         ];
        
//         // Benchmark password hashing
//         println!("\n1. Password Hashing (CPU-intensive):");
//         let start = Instant::now();
//         let mut hashes = Vec::new();
        
//         for password in &passwords {
//             let hash_start = Instant::now();
//             let hash = hash_password(password);
//             let hash_duration = hash_start.elapsed();
//             hashes.push(hash);
//             println!("  Hash '{}': {:?}", password, hash_duration);
//         }
        
//         let total_hashing_time = start.elapsed();
//         println!("  Total hashing time for {} passwords: {:?}", passwords.len(), total_hashing_time);
//         println!("  Average per password: {:?}", total_hashing_time / passwords.len() as u32);
        
//         // Benchmark password verification
//         println!("\n2. Password Verification (also CPU-intensive):");
//         let start = Instant::now();
        
//         for (i, password) in passwords.iter().enumerate() {
//             let verify_start = Instant::now();
//             let verified = verify_password(password, &hashes[i]);
//             let verify_duration = verify_start.elapsed();
//             println!("  Verify '{}': {:?} (result: {})", password, verify_duration, verified);
//         }
        
//         let total_verification_time = start.elapsed();
//         println!("  Total verification time for {} passwords: {:?}", passwords.len(), total_verification_time);
//         println!("  Average per verification: {:?}", total_verification_time / passwords.len() as u32);
        
//         // Compare with simple operations
//         println!("\n3. Simple String Operations (for comparison):");
//         let start = Instant::now();
        
//         for password in &passwords {
//             let simple_start = Instant::now();
//             let _length = password.len();
//             let _uppercase = password.to_uppercase();
//             let _contains_number = password.chars().any(|c| c.is_numeric());
//             let simple_duration = simple_start.elapsed();
//             println!("  Simple operations on '{}': {:?}", password, simple_duration);
//         }
        
//         let total_simple_time = start.elapsed();
//         println!("  Total simple operations time: {:?}", total_simple_time);
        
//         // Show the dramatic difference
//         println!("\n=== Performance Comparison ===");
//         println!("Password hashing: {:?}", total_hashing_time);
//         println!("Simple operations: {:?}", total_simple_time);
//         println!("Hashing is {:.1}x slower than simple operations", 
//                 total_hashing_time.as_nanos() as f64 / total_simple_time.as_nanos() as f64);
        
//         // CPU usage demonstration
//         println!("\n=== CPU Usage Demonstration ===");
//         println!("Notice how password hashing takes significantly longer because it:");
//         println!("1. Uses memory-intensive operations");
//         println!("2. Performs multiple computational rounds");
//         println!("3. Generates random salt for each hash");
//         println!("4. Designed to be slow to prevent brute force attacks");
//     }
// }
