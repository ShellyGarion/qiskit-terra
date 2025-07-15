use rand::Rng;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CliffordGate {
    CNOT(usize, usize),
    CZ(usize, usize),
    H(usize),
    S(usize),
    Sd(usize),
    SqrtX(usize),
    SqrtXd(usize),
}
impl CliffordGate {
    pub fn dagger(&self) -> Self {
        match self {
            Self::S(i) => Self::Sd(*i),
            Self::SqrtX(i) => Self::SqrtXd(*i),
            Self::Sd(i) => Self::S(*i),
            Self::SqrtXd(i) => Self::SqrtX(*i),
            _ => *self,
        }
    }
    pub fn to_vec(&self) -> (String, Vec<usize>) {
        match self {
            CliffordGate::CNOT(i, j) => ("CNOT".to_owned(), vec![*i, *j]),
            CliffordGate::CZ(i, j) => ("CZ".to_owned(), vec![*i, *j]),
            CliffordGate::H(i) => ("H".to_owned(), vec![*i]),
            CliffordGate::S(i) => ("S".to_owned(), vec![*i]),
            CliffordGate::Sd(i) => ("Sd".to_owned(), vec![*i]),
            CliffordGate::SqrtX(i) => ("SqrtX".to_owned(), vec![*i]),
            CliffordGate::SqrtXd(i) => ("SqrtXd".to_owned(), vec![*i]),
        }
    }
    pub fn from_vec(gate: &str, qbits: &[usize]) -> Self {
        match gate {
            "H" => Self::H(qbits[0]),
            "S" => Self::S(qbits[0]),
            "Sd" => Self::Sd(qbits[0]),
            "SqrtX" => Self::SqrtX(qbits[0]),
            "SqrtXd" => Self::SqrtXd(qbits[0]),
            "CX" => Self::CNOT(qbits[0], qbits[1]),
            "CNOT" => Self::CNOT(qbits[0], qbits[1]),
            "CZ" => Self::CZ(qbits[0], qbits[1]),
            _ => panic!("Unknown gate {}", gate),
        }
    }
    pub fn arity(&self) -> usize {
        match self {
            CliffordGate::CNOT(_, _) => 2,
            CliffordGate::CZ(_, _) => 2,
            _ => 1,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CliffordCircuit {
    pub nqbits: usize,
    pub gates: Vec<CliffordGate>,
}

impl CliffordCircuit {
    pub fn new(nqbits: usize) -> Self {
        Self {
            nqbits,
            gates: Vec::new(),
        }
    }
    pub fn from_vec(gates: Vec<(String, Vec<usize>)>) -> Self {
        let mut nqbits = 0;
        for (_, qbits) in gates.iter() {
            for qbit in qbits {
                if qbit + 1 > nqbits {
                    nqbits = qbit + 1;
                }
            }
        }
        Self {
            nqbits,
            gates: gates
                .iter()
                .map(|(gate, qbits)| CliffordGate::from_vec(gate, qbits))
                .collect(),
        }
    }

    pub fn random(nqubits: usize, ngates: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut circuit = Self::new(nqubits);
        for _ in 0..ngates {
            if rng.gen_bool(0.5) {
                // CNOT
                let i = rng.gen_range(0..nqubits);
                let mut j = rng.gen_range(0..nqubits);
                while j == i {
                    j = rng.gen_range(0..nqubits);
                }
                circuit.gates.push(CliffordGate::CNOT(i, j));
                continue;
            }
            if rng.gen_bool(0.5) {
                // H
                let i = rng.gen_range(0..nqubits);
                circuit.gates.push(CliffordGate::H(i));
                continue;
            }
            let i = rng.gen_range(0..nqubits);
            circuit.gates.push(CliffordGate::S(i));
        }
        circuit
    }

    pub fn extend_with(&mut self, other: &CliffordCircuit) {
        self.gates.extend_from_slice(&other.gates);
    }
    /// Counts the number of CNOT gates
    pub fn cnot_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| matches!(gate, CliffordGate::CNOT(_, _)))
            .count()
    }
    /// Counts the number of CNOT gates
    pub fn entangling_count(&self) -> usize {
        self.gates
            .iter()
            .filter(|gate| matches!(gate, CliffordGate::CNOT(_, _) | CliffordGate::CZ(_, _)))
            .count()
    }
    /// Computes the CNOT depth of the circuit
    pub fn cnot_depth(&self) -> usize {
        let mut depths: Vec<usize> = vec![0; self.nqbits];
        for gate in self.gates.iter() {
            if let CliffordGate::CNOT(i, j) = gate {
                let gate_depth = std::cmp::max(depths[*i], depths[*j]) + 1;
                depths[*i] = gate_depth;
                depths[*j] = gate_depth;
            }
        }
        *depths.iter().max().unwrap()
    }
    /// Computes the CNOT depth of the circuit
    pub fn entangling_depth(&self) -> usize {
        let mut depths: Vec<usize> = vec![0; self.nqbits];
        for gate in self.gates.iter() {
            match gate {
                CliffordGate::CNOT(i, j) => {
                    let gate_depth = std::cmp::max(depths[*i], depths[*j]) + 1;
                    depths[*i] = gate_depth;
                    depths[*j] = gate_depth;
                }
                CliffordGate::CZ(i, j) => {
                    let gate_depth = std::cmp::max(depths[*i], depths[*j]) + 1;
                    depths[*i] = gate_depth;
                    depths[*j] = gate_depth;
                }
                _ => {}
            }
        }
        *depths.iter().max().unwrap()
    }
    /// Returns the inverse of the circuit
    pub fn dagger(&self) -> Self {
        let new_gates = self.gates.iter().rev().map(|gate| gate.dagger()).collect();
        Self {
            nqbits: self.nqbits,
            gates: new_gates,
        }
    }
}
