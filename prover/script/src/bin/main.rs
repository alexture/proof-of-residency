use sp1_sdk::{include_elf, utils, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const REGEX_IO_ELF: &[u8] = include_elf!("prover-program");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Create a new stdin with d the input for the program.
    let mut stdin = SP1Stdin::new();

    let file_bytes = std::fs::read("fatura_net.pdf").unwrap();

    stdin.write(&file_bytes);

    // Generate the proof for the given program and input.
    let client = ProverClient::new();
    println!("Prover client created");
    let (pk, vk) = client.setup(REGEX_IO_ELF);
    println!("vk: {:?}", vk.bytes32());
    let mut proof = client.prove(&pk, stdin).run().expect("proving failed");

    // Read the output.
    let res = proof.public_values.read::<bool>();
    println!("res: {}", res);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Test a round trip of proof serialization and deserialization.
    proof
        .save("proof-with-pis.bin")
        .expect("saving proof failed");
    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client
        .verify(&deserialized_proof, &vk)
        .expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}
