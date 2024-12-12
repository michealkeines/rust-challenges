# Schnorr Zero-Knowledge DLOG Proof

This Rust program implements a **Schnorr Zero-Knowledge Discrete Logarithm (DLOG) Proof** using elliptic curve cryptography. The proof enables a prover (Alice) to convince a verifier (Bob) that she knows a secret \(x\) (a private key) associated with a public key \(y = x \cdot G\), without revealing \(x\).

---

## Key Features
1. **Prover (Alice)**: Proves knowledge of the private key \(x\) without disclosing it.
2. **Verifier (Bob)**: Verifies the proof using Alice's public key \(y\).
3. **Zero-Knowledge**: The proof reveals nothing about the private key \(x\).
4. **Efficient**: Leverages elliptic curve cryptography for performance.

---

## Example Scenario: Alice and Bob

### Story
Alice wants to prove to Bob that she knows the secret key \(x\) corresponding to a public key \(y = x \cdot G\) (where \(G\) is a known base point on the elliptic curve). However, she doesn’t want to reveal \(x\) to Bob.

---

### Workflow

1. **Setup**: 
   - Alice generates a secret scalar \(x\) (her private key).
   - She computes \(y = x \cdot G\) (her public key).

2. **Prove**:
   - Alice uses the program to generate a Schnorr proof (`t`, `s`) to convince Bob she knows \(x\).

3. **Verify**:
   - Bob verifies the proof using Alice’s public key \(y\), the known base point \(G\), and the provided proof.

---

### Example Code

```rust
fn main() {
    // Step 1: Setup
    let sid = "session_1"; // Session ID
    let pid = 42;          // Alice's Party ID

    // Alice generates a secret `x`
    let alice_secret_x = generate_random_scalar();
    println!("Alice's secret x: {:?}", alice_secret_x);

    // Alice computes her public key `y = x * G`
    let base_point = ProjectivePoint::GENERATOR;
    let alice_public_y = base_point * alice_secret_x;

    // Step 2: Alice proves knowledge of `x`
    let proof = DLogProof::prove(sid, pid, alice_secret_x, alice_public_y, base_point);
    println!("Alice sends proof to Bob:");
    println!("  t (Commitment): {:?}", proof.t.to_affine());
    println!("  s (Response): {:?}", proof.s);

    // Step 3: Bob verifies the proof
    let is_valid = proof.verify(sid, pid, alice_public_y, base_point);
    if is_valid {
        println!("Bob: Proof is valid! Alice knows the secret.");
    } else {
        println!("Bob: Proof is invalid! Alice might not know the secret.");
    }
}
