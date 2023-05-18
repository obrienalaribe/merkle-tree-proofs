# Polkadot Blockchain Academy

# Assignment #1

There are a few subtasks for this assignment:
1. [Capture the Flag](#capture-the-flag)
1. [Merkle Tree](#merkle-tree)
1. [Nash Solver](#nash-solver)

The header is linked to the [source](./src/) within, each file contains the entire sub-assignment to complete.
Complete them in any order you see fit, and as completely as possible before the [deadline](#deadline).

See also the [Testing](#testing) and [Submission and Grading](#submission-and-grading) sections below for important information.

---

## 1. [Capture the Flag](./src/ctf.rs)

Done **in class together** - the challenges will be released over time for everyone to start work at the same time together.

### Generate a Seed

Perhaps using https://docs.substrate.io/reference/command-line-tools/subkey/

First [install needed deps & nightly Rusttoolchain](https://docs.substrate.io/install/) then:

```sh
cargo install --force subkey --git https://github.com/paritytech/substrate --version 2.0.2 --locked

subkey generate -w 24
```

Or simply on online quick tool, like https://microbitcoinorg.github.io/mnemonic/

---

## 2. [Merkle Tree](./src/merkle.rs)

Your task is outlined in the code comments. 

---

## 3. [Nash Solver](./src/nash.rs)

**Task:** Code a Nash equilibrium solver for 2x2 matrix games.

The solver takes as input two 2x2 matrices of payoffs, $P_1$ and $P_2$, for the two players and returns the set of pure-strategy profiles.

For example, for the coordination game, the matrices are as follows:

$$P_1 = \begin{pmatrix} 1 & 0 \\\ 0 & 2 \end{pmatrix} \text{ and } P_2 = \begin{pmatrix} 1 & 0 \\\ 0 & 2 \end{pmatrix}$$

The solver should then return a list $N$ of action profiles that correspond to Nash equilibria.
If we encode the actions as $1$ and $2$, then the output should be $N = \{(1,1),(2,2)\}$, meaning that it is an equilibrium for both players to choose strategy $1$ and one for both players to choose strategy $2$.

If the game has no pure-strategy equilibrium, then the function should return an empty list.

### About the Code

The code provided here is a minimal harness to integrate with our Github Classroom.
It expects two payoff matrices in row major order and will provide these to you inside the `compute_equilibria` function.
You are only really expected to touch that function to implement your solution.

So to run the example from above you would execute:
```
cargo run "1 0 0 2" "1 0 0 2"
```

To get you started with some example inputs and expected outputs, we also provide some basic test cases that can be run with:

```
cargo test
```

### Your submission

Please document your code (in comments) with the rationals behind your solution and cite any external sources you used to come up with it.
If there a limits or caveats to your solution, please mention them as well.

If you can think of other 2x2 games to check the validity of your solver further, please add them to the set of tests as well.

---

## Testing

We have included integration tests so you can minimally check your work.
Passing all non-optional items in `/tests` is required to be considered for further more rigorous grading.
Failing tests that explicitly emit or are marked `OPTIONAL` is allowed.
Our provided tests are intentionally light.

You can compile and run an [integration test](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) for a specific problem by specifying the filename prefix, for example:

```sh
reset && cargo t --test ctf # test results exclusively `src/a_some_file.rs`
reset && cargo t --test merkle # test results exclusively `src/b_some_file.rs`
reset && cargo t --test nash # test results exclusively `src/c_some_file.rs`
```

**Do not modify the included integration tests, as grading requires them to remain unchanged.**

However, we recommend you write your own unit tests in your solutions to ensure your work is correct.

## Submission and Grading

Work will only be graded if pushed to the `main` branch in Github before the deadline, all other branches will be ignored.

### Deadline

The deadline for submission will be communicated when the assignment is first sent to you, and the Github classroom invitation link mentions this explicitly as well.
All grades will be assessed using the commit present on `main` at the time of the deadline.
All other work will be ignored.

### Private Test Suite and Manual Grading

The primary way we will grade your work is through an automated testing suite that is kept private to the Academy team.

There are also some human-graded aspects such as:

- Ensuring that your code is of high quality and readability.
- Ensuring that your solutions are not plagiarized.
- Ensuring that you haven't imported a crate to do the heavy lifting of your code problem.
- Ensuring that you have written the actual algorithm requested in a particular problem as opposed to a different algorithm that is easier to code.
  Eg writing bubble sort when we asked for merge sort.
- Ensuring you have followed any specific directions for a problem.
  Eg used functional-style iterator methods as opposed to imperative-style loops.

## 🚀 Good luck! 🚀

Please reach out to the Academy team if you have any questions or concerns related to this assignment.
# merkle-tree-proofs
