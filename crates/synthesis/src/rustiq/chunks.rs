use crate::rustiq::clifford_circuit::CliffordGate;

/// For each chunk (indexed by `c = 0..18`), each qubit (indexed by `q = 0..1`),
/// and each Pauli pair (indexed by `p = 0..16`), `CHUNK_CONJUGATION_SCORE[c][q][p]`
/// is 1 iff conjugating `p` by `c` is `I` on qubit `q`.
///
/// ->: II, IX, IY, IZ, XI, XX, XY, XZ, YI, YX, YY, YZ, ZI, ZX, ZY, ZZ,
///  0: II, IX, ZY, ZZ, XX, XI, YZ, YY, YX, YI, XZ, XY, ZI, ZX, IY, IZ,
///  1: II, XX, XY, IZ, XI, IX, IY, XZ, YZ, ZY, ZX, YI, ZZ, YY, YX, ZI,
///  2: II, ZZ, ZY, IX, XX, YY, YZ, XI, YX, XY, XZ, YI, ZI, IZ, IY, ZX,
///  3: II, XX, XY, IZ, ZZ, YY, YX, ZI, YZ, ZY, ZX, YI, XI, IX, IY, XZ,
///  4: II, ZY, IX, ZZ, XX, YZ, XI, YY, YX, XZ, YI, XY, ZI, IY, ZX, IZ,
///  5: II, XX, XY, IZ, YZ, ZY, ZX, YI, XI, IX, IY, XZ, ZZ, YY, YX, ZI,
///  6: II, IX, ZY, ZZ, ZI, ZX, IY, IZ, YX, YI, XZ, XY, XX, XI, YZ, YY,
///  7: II, IZ, XY, XX, XI, XZ, IY, IX, YZ, YI, ZX, ZY, ZZ, ZI, YX, YY,
///  8: II, ZZ, ZY, IX, ZI, IZ, IY, ZX, YX, XY, XZ, YI, XX, YY, YZ, XI,
///  9: II, IZ, XY, XX, ZZ, ZI, YX, YY, YZ, YI, ZX, ZY, XI, XZ, IY, IX,
/// 10: II, ZY, IX, ZZ, ZI, IY, ZX, IZ, YX, XZ, YI, XY, XX, YZ, XI, YY,
/// 11: II, IZ, XY, XX, YZ, YI, ZX, ZY, XI, XZ, IY, IX, ZZ, ZI, YX, YY,
/// 12: II, IX, ZY, ZZ, XX, XI, YZ, YY, ZI, ZX, IY, IZ, YX, YI, XZ, XY,
/// 13: II, XX, IZ, XY, XI, IX, XZ, IY, YZ, ZY, YI, ZX, ZZ, YY, ZI, YX,
/// 14: II, ZZ, ZY, IX, XX, YY, YZ, XI, ZI, IZ, IY, ZX, YX, XY, XZ, YI,
/// 15: II, XX, IZ, XY, ZZ, YY, ZI, YX, YZ, ZY, YI, ZX, XI, IX, XZ, IY,
/// 16: II, ZY, IX, ZZ, XX, YZ, XI, YY, ZI, IY, ZX, IZ, YX, XZ, YI, XY,
/// 17: II, XX, IZ, XY, YZ, ZY, YI, ZX, XI, IX, XZ, IY, ZZ, YY, ZI, YX,
pub static CHUNK_CONJUGATION_SCORE: [[[usize; 16]; 2]; 18] = [
    [
        [1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
    ],
    [
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
    ],
    [
        [1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
    ],
    [
        [1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
    ],
    [
        [1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    ],
    [
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    ],
    [
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0],
    ],
    [
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0],
    ],
    [
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
    ],
    [
        [1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
    ],
    [
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    ],
    [
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
    ],
    [
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
    ],
    [
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    ],
    [
        [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    ],
    [
        [1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
    ],
    [
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0],
    ],
    [
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0],
    ],
];
