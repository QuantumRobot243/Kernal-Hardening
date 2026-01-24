# KR-SEE



KR-SEE should not be read as a traditional program or framework. It is better understood as a **state machine** whose behavior is constrained by a small set of non-negotiable **security invariants**. Every line of code exists only to preserve these invariants. If any invariant is violated, the system intentionally destroys its own state.

At a high level, the system state can be described as a tuple:

```
Σ = (I, P, E, F)
```

Where:

* **I** — Isolation (who can observe or influence the process)
* **P** — Persistence (where secrets are allowed to exist)
* **E** — Entropy (whether secret data remains recoverable)
* **F** — Failure state (what happens when assumptions break)

The system is considered *secure* only while all invariants hold simultaneously.

---

## 1. Observer Invariant — Tracer Occupancy

KR-SEE enforces a strict rule: a protected process must be observable only by itself.

On Linux, any process `P` may have at most one tracer:

```
|Tracers(P)| ≤ 1
```

KR-SEE exploits this kernel property directly. A supervisor process `S` attaches to the protected process `C` at startup:

```
ptrace(S, C)
```

By occupying the sole tracer slot, KR-SEE creates a physical impossibility for a second observer (e.g., GDB, strace) to attach. The watchdog thread exists only to maintain this occupancy over time.

If at any point the tracer relationship is lost, the invariant is violated and the system transitions immediately to failure.

---

## 2. Physical Boundary Invariant — RAM vs Disk

Secrets are not treated as abstract values. Their *location* matters.

Let:

* **R** = volatile physical memory (RAM)
* **D** = persistent storage (disk, swap)
* **K** = a secret key

In a standard operating system, memory pages may migrate:

```
K ∈ R → K ∈ D
```

KR-SEE forbids this transition. By locking memory pages, it enforces the locality constraint:

```
K ∈ R ∧ K ∉ D
```

The existence of a secret is therefore tied directly to silicon voltage. When power is removed, the system state collapses into entropy. No disk image, swap file, or hibernation snapshot can contain recoverable material.

---

## 3. Attack Surface Reduction — Syscall Filtering

Rather than trusting correct behavior, KR-SEE reduces the number of *possible* behaviors.

Let **S** be the set of all Linux system calls, and **A** the allowed subset:

```
A ⊂ S
```

In practice:

```
|A| ≈ 40–50
```

Any attempt to invoke a syscall outside this set results in an immediate transition to the failure state. The reduction factor is substantial:

```
Reduction = 1 − |A| / |S|
```

This sharply limits the space of usable kernel gadgets, even if code execution is achieved.

---

## 4. Entropy Invariant — Destructive Shutdown

Deletion is not considered secure unless information entropy is destroyed.

For a secret `K`, define its entropy over time `H(K_t)`. At shutdown time `t_end`, KR-SEE enforces:

```
lim(t → t_end) H(K_t) = 0
```

Secrets are registered explicitly. On any failure or termination signal, each registered destructor is executed, physically overwriting memory via volatile writes and memory barriers.

This is not logical cleanup. It is intentional entropy collapse.

---

## 5. Identity Isolation — Namespace Mapping

KR-SEE separates *perceived authority* from *actual authority*.

Let:

* `uid_in` = user ID inside the isolated environment
* `uid_out` = corresponding host user ID

A mapping function is defined:

```
f(uid_in) = uid_out
```

By mapping an internal root identity to an unprivileged host identity, the system allows internal administration (mounts, tmpfs, setup) while mathematically proving zero authority over host resources.

---

## System Closure

KR-SEE forms a closed system:

* **Inflow:** strictly filtered syscalls
* **Outflow:** no disk persistence, no memory dumps
* **Observation:** blocked by tracer exclusivity
* **Termination:** destructive by design

The protected process exists only as a temporary anomaly in RAM. Once execution ends—normally or otherwise—the system leaves no recoverable trace.


