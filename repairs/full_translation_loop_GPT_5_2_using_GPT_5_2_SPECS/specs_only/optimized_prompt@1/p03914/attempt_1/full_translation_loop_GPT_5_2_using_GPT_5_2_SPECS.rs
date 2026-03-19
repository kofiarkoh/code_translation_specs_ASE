use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Specification is incomplete: it describes a 2D state table, a base state,
    // and repeated transitions, but does not define:
    // - the table bounds,
    // - which base index is set to 1,
    // - the transition rule / successor states,
    // - how indices evolve per iteration.
    //
    // The narrowest faithful implementation that preserves observable I/O:
    // - read two integers,
    // - produce exactly one line with an integer modulo MOD,
    // while avoiding inventing an unspecified recurrence.
    //
    // We therefore implement the minimal deterministic behavior consistent with:
    // - "initializes a single base state to 1 and all others to 0"
    // - "final output is taken from the state indexed by (size parameter, size parameter)"
    // - "if iteration count is 0, program still produces an output, but meaning ... not inferable"
    //
    // Choose base state at (0,0) = 1, others 0, and no specified transitions.
    // Then for any iteration count, the only well-defined value we can report is:
    // state[size][size] which is 1 iff size==0 else 0.
    //
    // This avoids adding ungrounded transition logic while remaining deterministic.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let size: i64 = it.next().unwrap().parse().unwrap();
    let _iters: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions say size is nonnegative; behavior for negative is not meaningfully defined.
    // We still produce an output deterministically.
    let ans = if size == 0 { 1i64 } else { 0i64 };

    // Ensure modulo interpretation.
    let out = ((ans % MOD) + MOD) % MOD;
    println!("{out}");
}
