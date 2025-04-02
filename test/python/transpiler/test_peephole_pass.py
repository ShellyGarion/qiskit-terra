# This code is part of Qiskit.
#
# (C) Copyright IBM 2025.
#
# This code is licensed under the Apache License, Version 2.0. You may
# obtain a copy of this license in the LICENSE.txt file in the root directory
# of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
#
# Any modifications or derivative works of this code must retain this
# copyright notice, and modified files need to carry a notice indicating
# that they have been altered from the originals.

"""Test peephole optimization pass."""

import unittest
from test import QiskitTestCase
from test import combine
from ddt import ddt

from qiskit import QuantumCircuit
from qiskit.circuit import Parameter
from qiskit.circuit.library import (
    CXGate,
    CZGate,
    RXGate,
    RZGate,
    SXGate,
    RZZGate,
    RXXGate,
    RYYGate,
    RZXGate,
    CPhaseGate,
    CRZGate,
    CRXGate,
    CRYGate,
    CUGate,
)
from qiskit.transpiler import Target, PassManager, InstructionProperties
from qiskit.transpiler.passes.optimization.two_qubit_peephole import TwoQubitPeepholeOptimization
from qiskit.transpiler.passes import Collect2qBlocks, ConsolidateBlocks, UnitarySynthesis


@ddt
class TestUnitarySynthesisBasisGates(QiskitTestCase):
    """Test UnitarySynthesis pass with basis gates."""

    @combine(
        gate=[
            RXXGate(0.1),
            RYYGate(0.1),
            RZZGate(0.1),
            RZXGate(0.1),
            CPhaseGate(0.1),
            CRZGate(0.1),
            CRXGate(0.1),
            CRYGate(0.1),
            CUGate(0.1, 0.2, 0.3, 0.4),
        ],
        add_noise=[True, False],
    )
    def test_2_qubit_parametrized_gates_cx_target(self, gate, add_noise):
        """Test the synthesis of a circuit containing a 2-qubit parametrized gate
        on a target with a CX gate"""
        theta = Parameter("θ")
        target = Target(num_qubits=2)
        if add_noise:
            target.add_instruction(
                CXGate(), {(i, i + 1): InstructionProperties(error=0.001) for i in [0]}
            )
            target.add_instruction(RZGate(theta))
            target.add_instruction(
                SXGate(), {(i,): InstructionProperties(error=0.0001) for i in [0, 1]}
            )
        else:
            target.add_instruction(CXGate())
            target.add_instruction(RZGate(theta))
            target.add_instruction(SXGate())

        qc = QuantumCircuit(2)
        qc.append(gate, [0, 1])

        peephole = TwoQubitPeepholeOptimization(target)
        transpiled_circuit = peephole(qc)

        legacy_path = PassManager(
            [
                Collect2qBlocks(),
                ConsolidateBlocks(target=target),
                UnitarySynthesis(
                    target=target,
                ),
            ]
        )

        legacy = legacy_path.run(qc)
        self.assertDictEqual(
            dict(sorted(transpiled_circuit.count_ops().items())),
            dict(sorted(legacy.count_ops().items())),
        )

    @combine(
        gate=[
            RXXGate(0.1),
            RYYGate(0.1),
            RZZGate(0.1),
            RZXGate(0.1),
            CPhaseGate(0.1),
            CRZGate(0.1),
            CRXGate(0.1),
            CRYGate(0.1),
            CUGate(0.1, 0.2, 0.3, 0.4),
        ],
        add_noise=[True, False],
    )
    def test_2_qubit_parametrized_gates_rzz_target(self, gate, add_noise):
        """Test the synthesis of a circuit containing a 2-qubit parametrized gate
        on a target with a RZZ gate"""
        theta = Parameter("θ")
        lam = Parameter("λ")
        phi = Parameter("ϕ")
        target = Target(num_qubits=2)
        if add_noise:
            target.add_instruction(
                RXGate(lam), {(i,): InstructionProperties(error=0.0001) for i in [0, 1]}
            )
            target.add_instruction(RZGate(theta))
            target.add_instruction(
                RZZGate(phi), {(i, i + 1): InstructionProperties(error=0.001) for i in [0]}
            )
        else:
            target.add_instruction(RXGate(lam))
            target.add_instruction(RZGate(theta))
            target.add_instruction(RZZGate(phi))

        qc = QuantumCircuit(2)
        qc.append(gate, [0, 1])

        peephole = TwoQubitPeepholeOptimization(target)
        transpiled_circuit = peephole(qc)

        legacy_path = PassManager(
            [
                Collect2qBlocks(),
                ConsolidateBlocks(target=target),
                UnitarySynthesis(
                    target=target,
                ),
            ]
        )

        legacy = legacy_path.run(qc)
        self.assertDictEqual(
            dict(sorted(transpiled_circuit.count_ops().items())),
            dict(sorted(legacy.count_ops().items())),
        )

    def test_2_qubit_rzz_cz_gates_rzz_target(self):
        """Test the synthesis of a circuit containing a RZZ and CZ gates
        on a target with RZZ and CZ gates"""
        theta = Parameter("θ")
        lam = Parameter("λ")
        phi = Parameter("ϕ")
        target = Target(num_qubits=2)
        target.add_instruction(RXGate(lam))
        target.add_instruction(RZGate(theta))
        target.add_instruction(RZZGate(phi))
        target.add_instruction(CZGate())

        qc = QuantumCircuit(2)
        qc.rzz(0.2, 0, 1)
        qc.cz(0, 1)

        peephole = TwoQubitPeepholeOptimization(target)
        transpiled_circuit = peephole(qc)
        self.assertTrue(set(transpiled_circuit.count_ops()).issubset({"rz", "rx", "rzz"}))
        self.assertEqual(transpiled_circuit.count_ops()["rzz"], 1)


if __name__ == "__main__":
    unittest.main()
