# This code is part of Qiskit.
#
# (C) Copyright IBM 2017, 2022.
#
# This code is licensed under the Apache License, Version 2.0. You may
# obtain a copy of this license in the LICENSE.txt file in the root directory
# of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
#
# Any modifications or derivative works of this code must retain this
# copyright notice, and modified files need to carry a notice indicating
# that they have been altered from the originals.

"""Tests for Clifford class."""

import unittest
from test import combine
from ddt import ddt

import numpy as np

from qiskit.test import QiskitTestCase
from qiskit.quantum_info.operators import Clifford
from qiskit.quantum_info import random_clifford
from qiskit.synthesis.clifford.clifford_decompose_layers import (
    _default_cx_synth_func,
    _default_cz_synth_func,
    synth_clifford_layers,
)


@ddt
class TestCliffordDecomposeLayers(QiskitTestCase):
    """Tests for clifford advanced decomposition functions."""

    @combine(num_qubits=[4, 5, 6, 7])
    def test_decompose_clifford(self, num_qubits):
        """Create layer decomposition for a Clifford U, and check that it
        results in an equivalent Clifford."""
        rng = np.random.default_rng(1234)
        samples = 10
        for _ in range(samples):
            cliff = random_clifford(num_qubits, seed=rng)
            circ = synth_clifford_layers(
                cliff, _default_cx_synth_func, _default_cz_synth_func, validate=True
            )
            cliff_target = Clifford(circ)
            self.assertEqual(cliff, cliff_target)


if __name__ == "__main__":
    unittest.main()