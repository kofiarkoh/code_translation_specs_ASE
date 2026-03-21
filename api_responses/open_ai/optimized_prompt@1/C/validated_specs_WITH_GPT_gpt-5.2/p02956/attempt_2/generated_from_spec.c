#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 998244353

typedef struct {
    int key; /* used for sorting/grouping; initially a, then b, then BRank, then AID */
    int id;  /* associated label; initially b, then AID, then AID, then BRank */
} Node;

static int cmp_key(const void *pa, const void *pb) {
    const Node *a = (const Node *)pa;
    const Node *b = (const Node *)pb;
    if (a->key < b->key) return -1;
    if (a->key > b->key) return 1;
    return 0;
}

static void bit_add(int *bit, int n, int idx, int delta) {
    for (; idx <= n; idx += idx & -idx) bit[idx] += delta;
}

static int bit_sum(const int *bit, int idx) {
    int s = 0;
    for (; idx > 0; idx -= idx & -idx) s += bit[idx];
    return s;
}

static long long mod_norm(long long x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) return 0;

    Node *d = (Node *)malloc((size_t)N * sizeof(Node));
    if (!d) return 0;

    for (int i = 0; i < N; i++) {
        int a, b;
        scanf("%d %d", &a, &b);
        d[i].key = a;
        d[i].id = b;
    }

    /* 5.1 Compress a into AID: sort by a, group equal a, assign AID; store (key=b, id=AID). */
    qsort(d, (size_t)N, sizeof(Node), cmp_key);
    int m = 0;
    for (int i = 0; i < N; ) {
        int j = i + 1;
        while (j < N && d[j].key == d[i].key) j++;
        m++;
        for (int k = i; k < j; k++) {
            int orig_b = d[k].id;
            d[k].key = orig_b;
            d[k].id = m;
        }
        i = j;
    }

    /* 5.2 Compress b into BRank: sort by b (currently key), group equal, assign BRank; store (key=BRank, id=AID). */
    qsort(d, (size_t)N, sizeof(Node), cmp_key);
    int br = 0;
    for (int i = 0; i < N; ) {
        int j = i + 1;
        while (j < N && d[j].key == d[i].key) j++;
        br++;
        for (int k = i; k < j; k++) d[k].key = br;
        i = j;
    }

    /* 6 pow[k] = 2^k mod MOD */
    int *pow2 = (int *)malloc((size_t)(N + 1) * sizeof(int));
    if (!pow2) {
        free(d);
        return 0;
    }
    pow2[0] = 1;
    for (int i = 1; i <= N; i++) {
        pow2[i] = (int)((2LL * pow2[i - 1]) % MOD);
    }

    /* 7 ans = ((pow[N] - 1) * N) mod MOD */
    long long ans = (mod_norm((long long)pow2[N] - 1) * (long long)N) % MOD;

    /* BIT over AID indices 1..m */
    int *bit = (int *)calloc((size_t)(m + 2), sizeof(int));
    if (!bit) {
        free(pow2);
        free(d);
        return 0;
    }

    /* 8.1 Forward sweep grouped by BRank (array already sorted by BRank). */
    for (int i = 0; i < N; ) {
        int j = i + 1;
        while (j < N && d[j].key == d[i].key) j++;

        for (int k = i; k < j; k++) {
            int id = d[k].id; /* AID */
            int L = bit_sum(bit, id - 1);
            int s_id = bit_sum(bit, id); /* prefix up to id */
            int exp2 = i - s_id;         /* as specified: i - sum_BIT(id) */
            ans += pow2[L];
            ans += pow2[exp2];
            ans %= MOD;
        }

        ans -= (long long)pow2[i] * (long long)(j - i) % MOD;
        ans %= MOD;

        for (int k = i; k < j; k++) bit_add(bit, m, d[k].id, 1);

        i = j;
    }

    /* 8.2 Backward sweep grouped by BRank. */
    memset(bit, 0, (size_t)(m + 2) * sizeof(int));
    for (int i = N - 1; i >= 0; ) {
        int j = i - 1;
        while (j >= 0 && d[j].key == d[i].key) j--;
        /* group is (j, i] => indices j+1..i */
        for (int k = i; k > j; k--) {
            int id = d[k].id; /* AID */
            int processed_right = N - i - 1;
            int L = bit_sum(bit, id - 1);
            int s_id = bit_sum(bit, id);
            int exp2 = processed_right - s_id;
            ans += pow2[L];
            ans += pow2[exp2];
            ans %= MOD;
        }

        ans -= (long long)pow2[N - i - 1] * (long long)(i - j) % MOD;
        ans %= MOD;

        for (int k = i; k > j; k--) bit_add(bit, m, d[k].id, 1);

        i = j;
    }

    /* 9 Swap fields: (BRank, AID) -> (AID, BRank), then sort by new key (AID). */
    for (int i = 0; i < N; i++) {
        int tmp = d[i].key;
        d[i].key = d[i].id;
        d[i].id = tmp;
    }
    qsort(d, (size_t)N, sizeof(Node), cmp_key);

    /* 9.1 Forward correction grouped by equal AID (key). */
    for (int i = 0; i < N; ) {
        int j = i + 1;
        while (j < N && d[j].key == d[i].key) j++;
        ans -= (long long)pow2[i] * (long long)(j - i) % MOD;
        ans %= MOD;
        i = j;
    }

    /* 9.2 Backward correction grouped by equal AID (key). */
    for (int i = N - 1; i >= 0; ) {
        int j = i - 1;
        while (j >= 0 && d[j].key == d[i].key) j--;
        ans -= (long long)pow2[N - i - 1] * (long long)(i - j) % MOD;
        ans %= MOD;
        i = j;
    }

    ans = mod_norm(ans);
    printf("%lld\n", ans);
    fflush(stdout);

    free(bit);
    free(pow2);
    free(d);
    return 0;
}
