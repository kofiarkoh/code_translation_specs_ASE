#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

/*
  We must compute the minimum number of operations to transform initial -> target
  using only:
    (1) x := x + 1
    (2) x := -x

  Constant time/memory: derive a closed-form minimum.

  Any sequence can be rearranged to at most one sign flip:
    - Multiple flips cancel in pairs; only parity matters.
    - Increments can be applied before/after the (optional) single flip.

  Therefore the minimum is:
    min( steps without flip, steps with one flip )

  Without flip: only increments, so reachable iff target >= initial, cost = target - initial.
  With one flip: choose k increments before flip, m after:
      start a
      after k increments: a + k
      flip: -(a + k)
      after m increments: -(a + k) + m = b
      => m = b + a + k
      with k >= 0, m >= 0 integers, cost = k + 1 + m = 1 + (b + a) + 2k
    Minimize over k subject to m >= 0:
      b + a + k >= 0  =>  k >= -(a + b)
      also k >= 0
      => k_min = max(0, -(a + b))
      cost_flip = 1 + (a + b) + 2*k_min

  Use int64_t for arithmetic; specification says result fits in signed 64-bit.
  Input parsing uses scanf; invalid input behavior is unspecified by spec.
*/

static int64_t max_i64(int64_t x, int64_t y) { return x > y ? x : y; }

int main(void) {
    int64_t a, b;
    if (scanf("%" SCNd64 " %" SCNd64, &a, &b) != 2) {
        /* Unspecified behavior for invalid input; exit quietly. */
        return 0;
    }

    int64_t best = INT64_MAX;

    /* Option 1: no sign flip (only increments). */
    if (b >= a) {
        int64_t cost_no_flip = b - a;
        if (cost_no_flip < best) best = cost_no_flip;
    }

    /* Option 2: exactly one sign flip. */
    {
        int64_t s = a + b;
        int64_t kmin = max_i64(0, -s);
        int64_t cost_flip = 1 + s + 2 * kmin;
        if (cost_flip < best) best = cost_flip;
    }

    /* best is guaranteed non-negative by construction. */
    printf("%" PRId64 "\n", best);
    return 0;
}
