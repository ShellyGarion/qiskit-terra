from qiskit.circuit import QuantumRegister, QuantumCircuit
from qiskit.quantum_info.operators.symplectic import Clifford
from qiskit import transpile

# This example runs through to the end
def experiment1():
    # create a new clifford (from a circuit)
    qc = QuantumCircuit(3)
    qc.s(0)
    qc.cx(0, 1)
    qc.h(1)
    print(qc)
    cliff = Clifford(qc)
    print(cliff)
    print(f"Created cliff of type {type(cliff)}")

    # append our clifford to another circuit
    q2 = QuantumRegister(5, "q")
    qc2 = QuantumCircuit(q2)
    qc2.h(0)
    qc2.h(1)
    qc2.barrier()
    qc2.append(cliff, [q2[0], q2[1], q2[4]], [])
    qc2.barrier()
    qc2.cx(3, 4)

    # draw the circuit (with our clifford inside)
    print(qc2)

    # decompose the circuit
    # (and only now decomposing our clifford)
    qc3 = qc2.decompose()

    # draw the decomposed circuit
    print(qc3)


# This example is still not fully working
def experiment2():
    # create a new clifford (from a circuit)
    qc = QuantumCircuit(3)
    qc.s(0)
    qc.cx(0, 1)
    qc.h(1)
    print(qc)
    cliff = Clifford(qc)
    print(cliff)
    print(f"Created cliff of type {type(cliff)}")

    # append our clifford to another circuit
    q2 = QuantumRegister(5, "q")
    qc2 = QuantumCircuit(q2)
    qc2.h(0)
    qc2.h(1)
    qc2.barrier()
    qc2.append(cliff, [q2[0], q2[1], q2[4]], [])
    qc2.barrier()
    qc2.cx(3, 4)

    # draw the circuit (with our clifford inside)
    print(qc2)

    # transpile the circuit (work-in-progress)
    # (and only now decomposing our clifford)
    qc3 = transpile(qc2, basis_gates={'rz', 'x', 'sx', 'cx'})

    # draw the decomposed circuit
    print(qc3)


# main
experiment1()