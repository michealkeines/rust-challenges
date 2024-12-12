// Required imports from the `k256` library for elliptic curve operations
use k256::{elliptic_curve::{group::GroupEncoding, ops::Reduce}, ProjectivePoint, Scalar, U256};

// Importing random number generation utility
use rand::thread_rng;

// Importing hashing functionality
use sha2::{Digest, Sha256};

/// A struct to encapsulate the Schnorr ZK DLOG (Zero-Knowledge Discrete Logarithm) proof.
/// The proof consists of two components:
/// 1. `t` - Commitment (a point on the elliptic curve)
/// 2. `s` - Response (a scalar value derived from the commitment and challenge)
#[derive(Debug, PartialEq, Eq)]
pub struct DLogProof {
    pub t: ProjectivePoint, // Commitment
    pub s: Scalar,          // Response
}

impl DLogProof {
    /// A deterministic hash function to compute the challenge value `c`.
    /// The challenge depends on:
    /// - `sid`: Session ID (a string identifier for the proof context)
    /// - `pid`: Party ID (a unique identifier for the prover)
    /// - `points`: A list of elliptic curve points used to derive the challenge
    fn hash_points(sid: &str, pid: u64, points: &[ProjectivePoint]) -> Scalar {
        let mut hasher = Sha256::new(); // Initialize a SHA-256 hasher
        hasher.update(sid.as_bytes()); // Include the session ID
        hasher.update(pid.to_le_bytes()); // Include the party ID in little-endian format
        for point in points {
            hasher.update(point.to_bytes().clone()); // Include each point (as bytes)
        }
        let hash = hasher.finalize(); // Finalize the hash computation
        Scalar::reduce(U256::from_be_slice(&hash)) // Reduce the hash to fit in the scalar field
    }

    /// Method to generate a Schnorr proof that the prover knows the secret `x` such that `y = x * G`.
    /// - `sid`: Session ID
    /// - `pid`: Party ID
    /// - `x`: Secret scalar (private key)
    /// - `y`: Public key (`x * G`, where `G` is the base point)
    /// - `base_point`: The base point `G` of the elliptic curve
    pub fn prove(
        sid: &str,
        pid: u64,
        x: Scalar,
        y: ProjectivePoint,
        base_point: ProjectivePoint,
    ) -> Self {
        let r = Scalar::generate_vartime(&mut thread_rng()); // Generate a random scalar `r`
        let t = base_point * r; // Compute commitment `t = r * G`
        let c = Self::hash_points(sid, pid, &[base_point, y, t]); // Compute challenge `c`
        let s = r + c * x; // Compute response `s = r + c * x`
        Self { t, s } // Return the proof containing `t` and `s`
    }

    /// Method to verify the Schnorr proof
    /// - `sid`: Session ID
    /// - `pid`: Party ID
    /// - `y`: Public key (`x * G`)
    /// - `base_point`: The base point `G` of the elliptic curve
    pub fn verify(
        &self,
        sid: &str,
        pid: u64,
        y: ProjectivePoint,
        base_point: ProjectivePoint,
    ) -> bool {
        let c = Self::hash_points(sid, pid, &[base_point, y, self.t]); // Recompute challenge `c`
        let lhs = base_point * self.s; // Compute `s * G`
        let rhs = self.t + (y * c); // Compute `t + c * y`
        lhs == rhs // Verification succeeds if `s * G == t + c * y`
    }
}

/// Helper function to generate a random scalar
fn generate_random_scalar() -> Scalar {
    Scalar::generate_vartime(&mut thread_rng())
}

fn main() {
    let sid = "sid"; // Session ID
    let pid = 1; // Party ID

    // Generate a random secret scalar `x`
    let x = generate_random_scalar();
    println!("Secret x: {:?}", x);

    // Compute the public key `y = x * G`
    let base_point = ProjectivePoint::GENERATOR;
    let y = base_point * x;

    // Generate the proof
    let start_proof = std::time::Instant::now(); // Start timer for proof computation
    let proof = DLogProof::prove(sid, pid, x, y, base_point);
    println!(
        "Proof computation time: {} ms",
        start_proof.elapsed().as_millis()
    );

    // Display proof components
    println!("Proof t (x, y): {:?}", proof.t.to_affine());
    println!("Proof s: {:?}", proof.s);

    // Verify the proof
    let start_verify = std::time::Instant::now(); // Start timer for verification
    let result = proof.verify(sid, pid, y, base_point);
    println!(
        "Verify computation time: {} ms",
        start_verify.elapsed().as_millis()
    );

    // Output verification result
    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test case to verify a valid Schnorr proof
    #[test]
    fn test_valid_proof() {
        // Test inputs
        let sid = "test_sid";
        let pid = 12345;

        // Generate random secret and compute corresponding public key
        let x = generate_random_scalar();
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * x;

        // Generate and verify the proof
        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        assert!(proof.verify(sid, pid, y, base_point), "Valid proof should pass");
    }

    /// Test case with an invalid public key `y`
    #[test]
    fn test_invalid_proof_wrong_y() {
        let sid = "test_sid";
        let pid = 12345;

        // Generate random secret but use a mismatched public key
        let x = generate_random_scalar();
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * generate_random_scalar(); // Different scalar

        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        assert!(
            !proof.verify(sid, pid, y, base_point),
            "Proof with incorrect y should fail"
        );
    }

    /// Test case with an incorrect session ID
    #[test]
    fn test_invalid_proof_wrong_sid() {
        let sid = "test_sid";
        let wrong_sid = "wrong_sid";
        let pid = 12345;

        let x = generate_random_scalar();
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * x;

        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        assert!(
            !proof.verify(wrong_sid, pid, y, base_point),
            "Proof with incorrect SID should fail"
        );
    }

    /// Test case with an incorrect party ID
    #[test]
    fn test_invalid_proof_wrong_pid() {
        let sid = "test_sid";
        let pid = 12345;
        let wrong_pid = 54321;

        let x = generate_random_scalar();
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * x;

        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        assert!(
            !proof.verify(sid, wrong_pid, y, base_point),
            "Proof with incorrect PID should fail"
        );
    }

    /// Test case for edge case: secret scalar `x` is zero
    #[test]
    fn test_edge_case_zero_scalar() {
        let sid = "test_sid";
        let pid = 12345;

        let x = Scalar::ZERO; // Scalar zero
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * x;

        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        assert!(proof.verify(sid, pid, y, base_point), "Proof with zero scalar should pass");
    }

    /// Test case for edge case: maximum scalar value
    #[test]
    fn test_edge_case_max_scalar() {
        let sid = "test_sid";
        let pid = 12345;

        let max_scalar_bytes = [0xFF; 32]; // Maximum 256-bit scalar value
        let x = Scalar::reduce(U256::from_be_slice(&max_scalar_bytes)); // Reduce scalar to curve order
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * x;

        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        assert!(proof.verify(sid, pid, y, base_point), "Proof with max scalar should pass");
    }

    /// Test case to measure proof and verification timings
    #[test]
    fn test_proof_timing() {
        let sid = "timing_test";
        let pid = 67890;

        let x = generate_random_scalar();
        let base_point = ProjectivePoint::GENERATOR;
        let y = base_point * x;

        let start_proof = std::time::Instant::now();
        let proof = DLogProof::prove(sid, pid, x, y, base_point);
        let proof_time = start_proof.elapsed().as_millis();
        assert!(proof_time < 500, "Proof computation should be fast");

        let start_verify = std::time::Instant::now();
        let valid = proof.verify(sid, pid, y, base_point);
        let verify_time = start_verify.elapsed().as_millis();
        assert!(verify_time < 500, "Verification computation should be fast");
        assert!(valid, "Valid proof should pass verification");
    }
}
