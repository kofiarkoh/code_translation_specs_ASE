#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MOD 1000000007LL

static int64_t mod_pow(int64_t a, int64_t e) {
    int64_t r = 1 % MOD;
    a %= MOD;
    while (e > 0) {
        if (e & 1) r = (r * a) % MOD;
        a = (a * a) % MOD;
        e >>= 1;
    }
    return r;
}

static int64_t mod_inv(int64_t a) {
    /* MOD is prime; inverse exists for a != 0 mod MOD under stated constraints (n < MOD). */
    a %= MOD;
    if (a < 0) a += MOD;
    return mod_pow(a, MOD - 2);
}

typedef struct {
    int to;
    int next;
} Edge;

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* Fixed-capacity arrays are implied by the spec; we allocate dynamically but do not validate. */
    int m = (n > 0) ? (2 * (n - 1)) : 0;
    Edge *edges = (Edge *)malloc((size_t)(m > 0 ? m : 1) * sizeof(Edge));
    int *head = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int *parent = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int *order = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int *sub = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int *parEdge = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int64_t *dp = (int64_t *)malloc((size_t)(n + 1) * sizeof(int64_t));
    int64_t *ans = (int64_t *)malloc((size_t)(n + 1) * sizeof(int64_t));
    int64_t *fact = (int64_t *)malloc((size_t)(n + 1) * sizeof(int64_t));
    int64_t *invfact = (int64_t *)malloc((size_t)(n + 1) * sizeof(int64_t));

    for (int i = 1; i <= n; i++) head[i] = -1;

    int ec = 0;
    for (int i = 0; i < n - 1; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        edges[ec].to = v; edges[ec].next = head[u]; head[u] = ec++;
        edges[ec].to = u; edges[ec].next = head[v]; head[v] = ec++;
    }

    /* Precompute factorials and inverse factorials up to n. */
    fact[0] = 1;
    for (int i = 1; i <= n; i++) fact[i] = (fact[i - 1] * (int64_t)i) % MOD;
    invfact[n] = mod_inv(fact[n]);
    for (int i = n; i >= 1; i--) invfact[i - 1] = (invfact[i] * (int64_t)i) % MOD;

    if (n == 1) {
        /* Empty products are 1; output 1 mod MOD. */
        printf("1\n");
        free(edges); free(head); free(parent); free(order); free(sub); free(parEdge);
        free(dp); free(ans); free(fact); free(invfact);
        return 0;
    }

    /* Build parent array and traversal order with an explicit stack (root at 1). */
    int *stack = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int top = 0, ordsz = 0;
    stack[top++] = 1;
    parent[1] = 0;
    parEdge[1] = -1;

    while (top) {
        int v = stack[--top];
        order[ordsz++] = v;
        for (int e = head[v]; e != -1; e = edges[e].next) {
            int u = edges[e].to;
            if (u == parent[v]) continue;
            parent[u] = v;
            parEdge[u] = e;
            stack[top++] = u;
        }
    }

    /* Post-order DP to compute subtree sizes and dp[v] for rooted tree at 1.
       dp[v] = inv(sub[v]) * prod_{child c} ( dp[c] * invfact[sub[c]] )  (mod MOD)
       This dp is an internal rerooting-friendly aggregate; final answer uses dp[v] and fact[n]. */
    for (int i = ordsz - 1; i >= 0; i--) {
        int v = order[i];
        int sz = 1;
        int64_t prod = 1;
        for (int e = head[v]; e != -1; e = edges[e].next) {
            int u = edges[e].to;
            if (u == parent[v]) continue;
            sz += sub[u];
            int64_t term = dp[u];
            term = (term * invfact[sub[u]]) % MOD;
            prod = (prod * term) % MOD;
        }
        sub[v] = sz;
        dp[v] = (prod * mod_inv((int64_t)sz)) % MOD;
    }

    /* Rerooting DFS using stack; when moving root across edge v-u, update dp/sub for v and u.
       After rerooting, dp[x] corresponds to the rooted-at-x aggregate over all neighbors. */
    int *st_v = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int *st_p = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int stn = 0;
    st_v[stn] = 1; st_p[stn] = 0; stn++;

    while (stn) {
        int v = st_v[--stn];
        int p = st_p[stn];

        /* Compute final answer for v:
           ans[v] = fact[n] * dp[v] * invfact[n-1]  (mod MOD)
           This matches: n! / (prod s_i! * prod s_i) where s_i are component sizes after removing v.
           Reason: dp[v] = inv(n-1) * prod_i ( inv(s_i) * inv(s_i!) ) when rooted at v,
           and multiplying by n! / (n-1)! = n gives the required expression. */
        ans[v] = fact[n];
        ans[v] = (ans[v] * dp[v]) % MOD;
        ans[v] = (ans[v] * invfact[n - 1]) % MOD;

        for (int e = head[v]; e != -1; e = edges[e].next) {
            int u = edges[e].to;
            if (u == p) continue;

            /* Save current state to restore after processing child. */
            int sub_v = sub[v], sub_u = sub[u];
            int64_t dp_v = dp[v], dp_u = dp[u];

            /* Remove u's contribution from v. */
            int64_t contrib_u = dp[u];
            contrib_u = (contrib_u * invfact[sub[u]]) % MOD;
            int64_t inv_contrib_u = mod_inv(contrib_u);

            int64_t prod_v = (dp[v] * (int64_t)sub[v]) % MOD; /* dp[v] = prod_v * inv(sub[v]) */
            prod_v = (prod_v * inv_contrib_u) % MOD;

            sub[v] -= sub[u];
            dp[v] = (prod_v * mod_inv((int64_t)sub[v])) % MOD;

            /* Add v's (now as child) contribution into u. */
            int64_t contrib_v = dp[v];
            contrib_v = (contrib_v * invfact[sub[v]]) % MOD;

            int64_t prod_u = (dp[u] * (int64_t)sub[u]) % MOD;
            prod_u = (prod_u * contrib_v) % MOD;

            sub[u] = sub_u + sub[v];
            dp[u] = (prod_u * mod_inv((int64_t)sub[u])) % MOD;

            st_v[stn] = u; st_p[stn] = v; stn++;

            /* Restore state for next neighbor. */
            sub[v] = sub_v; sub[u] = sub_u;
            dp[v] = dp_v; dp[u] = dp_u;
        }
    }

    for (int v = 1; v <= n; v++) {
        int64_t out = ans[v] % MOD;
        if (out < 0) out += MOD;
        printf("%lld\n", (long long)out);
    }

    free(st_v); free(st_p); free(stack);
    free(edges); free(head); free(parent); free(order); free(sub); free(parEdge);
    free(dp); free(ans); free(fact); free(invfact);
    return 0;
}
