#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;

static ll llabs_ll(ll x) { return x < 0 ? -x : x; }

static ll gcd_ll(ll a, ll b) {
    a = llabs_ll(a);
    b = llabs_ll(b);
    while (b != 0) {
        ll t = a % b;
        a = b;
        b = t;
    }
    return a;
}

/* Nonnegative modulo for possibly-negative a, with positive mod m */
static ll mod_nonneg(ll a, ll m) {
    ll r = a % m;
    if (r < 0) r += m;
    return r;
}

/*
 * Hash table storing at most one interval per residue class V (0..D-1),
 * but implemented as buckets with chaining; insertion merges only with the
 * first matching interval found in that bucket, per specification.
 */
typedef struct Node {
    ll v;      /* residue class */
    ll l, r;   /* half-open interval [l, r) */
    struct Node *next;
} Node;

static size_t hash_v(ll v, size_t mask) {
    /* simple mix; mask is (cap-1) where cap is power of two */
    uint64_t x = (uint64_t)v;
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccdULL;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53ULL;
    x ^= x >> 33;
    return (size_t)x & mask;
}

int main(void) {
    ll N, X, D;
    if (scanf("%lld %lld %lld", &N, &X, &D) != 3) return 0;

    /* 5.1 special cases */
    if (X == 0 && D == 0) {
        printf("1\n");
        fflush(stdout);
        return 0;
    }
    if (D == 0) {
        /* treated as distinct kX for k=0..N */
        printf("%lld\n", N + 1);
        fflush(stdout);
        return 0;
    }
    if (X == 0 && D != 0) {
        /* N(N-1)/2 + 1 */
        ll ans = (N * (N - 1)) / 2 + 1;
        printf("%lld\n", ans);
        fflush(stdout);
        return 0;
    }

    /* 5.2 general case: normalize by gcd(|X|,|D|) */
    ll G = gcd_ll(X, D);
    if (G != 0) {
        X /= G;
        D /= G;
    }

    /* Ensure internal D > 0; adjust endpoints accordingly before flipping sign */
    if (D < 0) {
        /* Reverse the arithmetic progression: X + kD, k=0..N-1 becomes
           X' + k(-D) with X' = X + (N-1)D. This preserves the multiset of terms. */
        X = X + (N - 1) * D;
        D = -D;
    }

    /* Hash table capacity: power of two >= 4*(N+1) for low collision rate */
    size_t need = (size_t)(N + 1);
    size_t cap = 1;
    while (cap < need * 4) cap <<= 1;
    size_t mask = cap - 1;

    Node **buckets = (Node **)calloc(cap, sizeof(Node *));
    if (!buckets) return 0;

    /* Generate one interval per i and merge into hash table */
    for (ll i = 0; i <= N; i++) {
        /* For fixed i, sums are:
           sum = i*X + D * (sum of chosen indices), indices in [0..N-1].
           The index-sum ranges from min = 0+1+...+(i-1) to max = (N-i)+...+(N-1).
           Thus:
             minIdxSum = i*(i-1)/2
             maxIdxSum = i*(2N - i - 1)/2
           Achievable sums form contiguous arithmetic progression with step D.
           Represent as half-open [L, R) where points are { s ≡ V (mod D), L <= s < R }.
        */
        ll minIdxSum = (i * (i - 1)) / 2;
        ll maxIdxSum = (i * (2 * N - i - 1)) / 2;

        ll base = i * X;
        ll L = base + D * minIdxSum;
        ll R = base + D * (maxIdxSum + 1); /* half-open: include maxIdxSum */

        ll V = mod_nonneg(L, D);

        size_t h = hash_v(V, mask);
        Node *cur = buckets[h];
        Node *found = NULL;

        /* Merge against the first matching interval found in this bucket */
        while (cur) {
            if (cur->v == V) {
                found = cur;
                break;
            }
            cur = cur->next;
        }

        if (!found) {
            Node *nn = (Node *)malloc(sizeof(Node));
            if (!nn) return 0;
            nn->v = V;
            nn->l = L;
            nn->r = R;
            nn->next = buckets[h];
            buckets[h] = nn;
        } else {
            /* mergeable when l <= R and r >= L */
            if (found->l <= R && found->r >= L) {
                if (L < found->l) found->l = L;
                if (R > found->r) found->r = R;
            } else {
                /* Not mergeable: per spec, program maintains at most one stored interval
                   per residue in its internal structure; narrowest faithful behavior is
                   to keep the existing one unchanged and ignore this interval. */
            }
        }
    }

    /* Count lattice points in each stored interval */
    ll ans = 0;
    for (size_t i = 0; i < cap; i++) {
        Node *cur = buckets[i];
        while (cur) {
            /* (r-l)/D counts how many integers in [l,r) with residue v */
            ans += (cur->r - cur->l) / D;
            cur = cur->next;
        }
    }

    printf("%lld\n", ans);
    fflush(stdout);

    /* cleanup */
    for (size_t i = 0; i < cap; i++) {
        Node *cur = buckets[i];
        while (cur) {
            Node *nx = cur->next;
            free(cur);
            cur = nx;
        }
    }
    free(buckets);
    return 0;
}
