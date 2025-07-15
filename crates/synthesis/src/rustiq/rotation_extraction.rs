/// A simple function that expresses a given circuit as a sequence of Pauli rotations
/// followed by a final Clifford operator
use crate::rustiq::pauli_like::PauliLike;
use crate::rustiq::tableau::Tableau;

pub fn extract_rotations(
    circuit: &[(String, Vec<usize>)],
    nqubits: usize,
) -> (Vec<(bool, String)>, Tableau) {
    let mut clifford = Tableau::new(nqubits);
    let mut rotations = Vec::new();
    for (gate_name, qbits) in circuit.iter() {
        match gate_name.as_str() {
            "CX" => clifford.cnot(qbits[0], qbits[1]),
            "CZ" => clifford.cz(qbits[0], qbits[1]),
            "H" => clifford.h(qbits[0]),
            "S" => clifford.s(qbits[0]),
            "Sd" => clifford.sd(qbits[0]),
            "SqrtX" => clifford.sqrt_x(qbits[0]),
            "SqrtXd" => clifford.sqrt_xd(qbits[0]),
            "X" => {
                clifford.sqrt_x(qbits[0]);
                clifford.sqrt_x(qbits[0])
            }
            "Z" => {
                clifford.s(qbits[0]);
                clifford.s(qbits[0])
            }
            "Y" => {
                clifford.sqrt_x(qbits[0]);
                clifford.s(qbits[0]);
                clifford.s(qbits[0]);
                clifford.sqrt_xd(qbits[0]);
            }
            "RZ" => {
                rotations.push(clifford.get_inverse_z(qbits[0]));
            }
            _ => panic!("Unsupported gate {}", gate_name),
        }
    }
    (rotations, clifford)
}
