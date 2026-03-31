#include <stdio.h>
#include <stdlib.h>

#define MAXN 200000
#define MAXE (2 * MAXN)
#define MOD 1000000007LL

static int n;
static int head[MAXN + 5];
static int to[MAXE + 5];
static int nxt[MAXE + 5];
static int ecnt = 0;

static int parent_[MAXN + 5];
static int order_[MAXN + 5];
static int ord_sz = 0;

static int sub[MAXN + 5];          /* subtree sizes for a fixed root (1) */
static long long inv_[MAXN + 5];   /* modular inverses of 1..n */
static long long fact[MAXN + 5];   /* factorials */
static long long ans[MAXN + 5];    /* per-node answers */

static void add_edge(int u, int v) {
    ++ecnt;
    to[ecnt] = v;
    nxt[ecnt] = head[u];
    head[u] = ecnt;
}

static long long mod_pow(long long a, long long e) {
    long long r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1) r = (r * a) % MOD;
        a = (a * a) % MOD;
        e >>= 1;
    }
    return r;
}

int main(void) {
    if (scanf("%d", &n) != 1) return 0;

    for (int i = 1; i <= n; i++) head[i] = 0;

    for (int i = 0; i < n - 1; i++) {
        int u, v;
        if (scanf("%d %d", &u, &v) != 2) return 0;
        add_edge(u, v);
        add_edge(v, u);
    }

    /* Precompute factorials and inverses up to n. */
    fact[0] = 1;
    for (int i = 1; i <= n; i++) fact[i] = (fact[i - 1] * i) % MOD;

    inv_[1] = 1;
    for (int i = 2; i <= n; i++) {
        /* MOD is prime; i < MOD under stated bounds, so inverse exists. */
        inv_[i] = mod_pow((long long)i, MOD - 2);
    }

    /* Build parent and traversal order with an explicit stack (root at 1). */
    for (int i = 1; i <= n; i++) parent_[i] = 0;
    ord_sz = 0;

    int *stack = (int *)malloc((size_t)(n + 5) * sizeof(int));
    int sp = 0;
    stack[sp++] = 1;
    parent_[1] = -1;

    while (sp > 0) {
        int u = stack[--sp];
        order_[ord_sz++] = u;
        for (int e = head[u]; e; e = nxt[e]) {
            int v = to[e];
            if (v == parent_[u]) continue;
            if (parent_[v] != 0) continue; /* tree assumption; avoids revisits */
            parent_[v] = u;
            stack[sp++] = v;
        }
    }

    /* Compute subtree sizes for root=1 using reverse order. */
    for (int i = 1; i <= n; i++) sub[i] = 1;
    for (int i = ord_sz - 1; i >= 0; i--) {
        int u = order_[i];
        for (int e = head[u]; e; e = nxt[e]) {
            int v = to[e];
            if (v == parent_[u]) continue;
            sub[u] += sub[v];
        }
    }

    /* Compute product P(root=1) = prod_u sub_u (subtree sizes under root=1). */
    long long P1 = 1;
    for (int i = 1; i <= n; i++) {
        P1 = (P1 * (long long)sub[i]) % MOD;
    }

    /* Reroot DP:
       Let P(r) = prod_u size(subtree_u when rooted at r).
       When moving root across edge r--v where v is a child of r in current rooting:
         sub_v changes from s to n
         sub_r changes from n to n - s
         all other subtree sizes unchanged
       So:
         P(v) = P(r) * (n / s) * ((n - s) / n)  (mod MOD)
              = P(r) * (n - s) / s             (mod MOD)
       Division is modular inverse; s in [1..n-1], invertible under preconditions.
    */
    long long *P = (long long *)malloc((size_t)(n + 5) * sizeof(long long));
    P[1] = P1;

    /* Traverse from root=1 outward using the same order_ (parent-first). */
    for (int i = 0; i < ord_sz; i++) {
        int r = order_[i];
        for (int e = head[r]; e; e = nxt[e]) {
            int v = to[e];
            if (v == parent_[r]) continue;
            int s = sub[v]; /* size of v-subtree when rooted at r (same as root=1 orientation) */
            long long val = P[r];
            val = (val * (long long)(n - s)) % MOD;
            val = (val * inv_[s]) % MOD;
            P[v] = val;
        }
    }

    /* Answer for root r: n! / P(r) mod MOD. */
    for (int r = 1; r <= n; r++) {
        long long invP = mod_pow(P[r], MOD - 2); /* valid under preconditions */
        ans[r] = (fact[n] * invP) % MOD;
    }

    for (int i = 1; i <= n; i++) {
        printf("%lld\n", ans[i]);
    }

    free(stack);
    free(P);
    return 0;
}
