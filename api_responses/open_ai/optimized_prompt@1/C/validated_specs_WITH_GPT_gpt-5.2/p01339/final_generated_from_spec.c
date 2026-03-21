#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL

typedef struct {
    int *a;
    int n;
    int cap;
} Vec;

static void vec_push(Vec *v, int x) {
    if (v->n == v->cap) {
        int ncap = v->cap ? (v->cap * 2) : 4;
        int *na = (int*)realloc(v->a, (size_t)ncap * sizeof(int));
        /* Per spec: allocation failure => undefined behavior; no handling. */
        v->a = na;
        v->cap = ncap;
    }
    v->a[v->n++] = x;
}

/* Input: nonnegative decimal integers separated by non-digits. */
static int read_int(void) {
    int c;
    do {
        c = getchar();
        if (c == EOF) return -1;
    } while (c < '0' || c > '9');
    int x = 0;
    while (c >= '0' && c <= '9') {
        x = x * 10 + (c - '0');
        c = getchar();
        if (c == EOF) break;
    }
    return x;
}

static void dfs1(int v, Vec *g, unsigned char *vis, int *order, int *ordn) {
    vis[v] = 1;
    for (int i = 0; i < g[v].n; i++) {
        int to = g[v].a[i];
        if (!vis[to]) dfs1(to, g, vis, order, ordn);
    }
    order[(*ordn)++] = v;
}

static void dfs2(int v, Vec *gr, int *cmp, int cid) {
    cmp[v] = cid;
    for (int i = 0; i < gr[v].n; i++) {
        int to = gr[v].a[i];
        if (cmp[to] < 0) dfs2(to, gr, cmp, cid);
    }
}

int main(void) {
    int N = read_int();
    int M = read_int();
    if (N < 0 || M < 0) return 0; /* Narrow behavior on missing input. */

    Vec *g = (Vec*)calloc((size_t)N, sizeof(Vec));
    Vec *gr = (Vec*)calloc((size_t)N, sizeof(Vec));
    int *first_out = (int*)malloc((size_t)N * sizeof(int));
    for (int i = 0; i < N; i++) first_out[i] = -1;

    for (int e = 0; e < M; e++) {
        int s = read_int();
        int d = read_int();
        /* Input is 1-based; convert to 0-based. */
        s--; d--;
        vec_push(&g[s], d);
        vec_push(&gr[d], s);
        if (first_out[s] < 0) first_out[s] = d; /* first stored outgoing edge */
    }

    unsigned char *vis = (unsigned char*)calloc((size_t)N, 1);
    int *order = (int*)malloc((size_t)N * sizeof(int));
    int ordn = 0;
    for (int v = 0; v < N; v++) {
        if (!vis[v]) dfs1(v, g, vis, order, &ordn);
    }

    int *cmp = (int*)malloc((size_t)N * sizeof(int));
    for (int i = 0; i < N; i++) cmp[i] = -1;

    int k = 0;
    for (int idx = N - 1; idx >= 0; idx--) {
        int v = order[idx];
        if (cmp[v] < 0) {
            dfs2(v, gr, cmp, k);
            k++;
        }
    }

    int *nx = (int*)malloc((size_t)k * sizeof(int));
    for (int i = 0; i < k; i++) nx[i] = -1;

    /* Overwrite rule: vertices in increasing order; use first stored outgoing edge. */
    for (int i = 0; i < N; i++) {
        int j = first_out[i];
        if (j >= 0) {
            nx[cmp[i]] = cmp[j];
        }
    }

    long long *dp = (long long*)calloc((size_t)k, sizeof(long long));
    long long ans = 1;

    for (int i = 0; i < k; i++) {
        if (dp[i] != 0) dp[i] = (dp[i] + 1) % MOD;
        else dp[i] = 2;

        int t = nx[i];
        if (t < 0 || t == i) {
            ans = (ans * dp[i]) % MOD;
        } else {
            if (dp[t] != 0) dp[t] = (dp[t] * dp[i]) % MOD;
            else dp[t] = dp[i];
        }
    }

    printf("%lld\n", ans % MOD);

    /* No need to free for single-run program; leaving allocated memory is fine. */
    return 0;
}
