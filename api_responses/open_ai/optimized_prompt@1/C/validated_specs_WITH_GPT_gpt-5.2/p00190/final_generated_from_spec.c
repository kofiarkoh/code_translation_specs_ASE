#include <stdio.h>
#include <stdint.h>
#include <string.h>

#define PRECOMP_MAX_DEPTH 16
#define QUERY_MAX_DEPTH 4

/* Fixed-size internal storage; if exceeded, behavior is unspecified per spec. */
#define MAX_PRECOMP_STATES 2000000
#define MAX_QUERY_VISITED  200000

/* Adjacency list exactly as specified. */
static const uint8_t adj[13][4] = {
    {2, 255, 255, 255},      /* 0: {2} */
    {2, 5, 255, 255},        /* 1: {2,5} */
    {0, 1, 3, 6},            /* 2: {0,1,3,6} */
    {2, 7, 255, 255},        /* 3: {2,7} */
    {5, 255, 255, 255},      /* 4: {5} */
    {1, 4, 6, 9},            /* 5: {1,4,6,9} */
    {2, 5, 7, 10},           /* 6: {2,5,7,10} */
    {3, 6, 8, 11},           /* 7: {3,6,8,11} */
    {7, 255, 255, 255},      /* 8: {7} */
    {5, 10, 255, 255},       /* 9: {5,10} */
    {6, 9, 11, 12},          /* 10:{6,9,11,12} */
    {7, 10, 255, 255},       /* 11:{7,10} */
    {10, 255, 255, 255}      /* 12:{10} */
};
static const uint8_t adj_deg[13] = {1,2,4,2,1,4,4,4,1,2,4,2,1};

/* State encoding: 13 nibbles (4 bits each) packed into 64-bit.
   Intended domain is 0..15; behavior unspecified if violated. */
static inline uint64_t pack_state(const uint8_t a[13]) {
    uint64_t x = 0;
    for (int i = 0; i < 13; i++) {
        x |= ((uint64_t)(a[i] & 0xF)) << (4 * i);
    }
    return x;
}
static inline uint8_t get_nib(uint64_t x, int i) {
    return (uint8_t)((x >> (4 * i)) & 0xFULL);
}
static inline uint64_t set_nib(uint64_t x, int i, uint8_t v) {
    uint64_t mask = (uint64_t)0xFULL << (4 * i);
    x = (x & ~mask) | (((uint64_t)(v & 0xF)) << (4 * i));
    return x;
}
static inline uint64_t swap_nib(uint64_t x, int i, int j) {
    uint8_t vi = get_nib(x, i);
    uint8_t vj = get_nib(x, j);
    x = set_nib(x, i, vj);
    x = set_nib(x, j, vi);
    return x;
}

/* Find first two zeros in position order 0..12. Behavior unspecified if fewer than two. */
static inline void find_zeros(uint64_t s, uint8_t *z0, uint8_t *z1) {
    int found = 0;
    uint8_t a = 0, b = 0;
    for (uint8_t i = 0; i < 13; i++) {
        if (get_nib(s, i) == 0) {
            if (found == 0) a = i;
            else { b = i; break; }
            found++;
        }
    }
    *z0 = a;
    *z1 = b;
}

/* Hash table for precomputed distances: open addressing with linear probing. */
#define PRE_HASH_SIZE (1u<<22) /* 4,194,304 slots */
static uint64_t pre_keys[PRE_HASH_SIZE];
static uint8_t  pre_vals[PRE_HASH_SIZE];
static uint8_t  pre_used[PRE_HASH_SIZE];

static inline uint32_t hash64(uint64_t x) {
    /* Simple 64->32 mix; deterministic. */
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccdULL;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53ULL;
    x ^= x >> 33;
    return (uint32_t)x;
}

static inline void pre_insert(uint64_t key, uint8_t dist) {
    uint32_t h = hash64(key) & (PRE_HASH_SIZE - 1u);
    while (pre_used[h]) {
        if (pre_keys[h] == key) {
            /* Keep the smaller distance (BFS should already ensure first is minimal). */
            if (dist < pre_vals[h]) pre_vals[h] = dist;
            return;
        }
        h = (h + 1u) & (PRE_HASH_SIZE - 1u);
    }
    pre_used[h] = 1;
    pre_keys[h] = key;
    pre_vals[h] = dist;
}

static inline int pre_lookup(uint64_t key, uint8_t *out_dist) {
    uint32_t h = hash64(key) & (PRE_HASH_SIZE - 1u);
    while (pre_used[h]) {
        if (pre_keys[h] == key) {
            *out_dist = pre_vals[h];
            return 1;
        }
        h = (h + 1u) & (PRE_HASH_SIZE - 1u);
    }
    return 0;
}

/* Precompute exact distances from goal for all states within 16 moves. */
typedef struct {
    uint64_t s;
    uint8_t z0, z1;
    uint8_t d;
} Node;

static Node pre_queue[MAX_PRECOMP_STATES];

static void precompute(void) {
    uint8_t goal_arr[13] = {1,2,3,4,5,6,7,8,9,10,11,0,0};
    uint64_t goal = pack_state(goal_arr);

    uint8_t gz0, gz1;
    find_zeros(goal, &gz0, &gz1);

    /* BFS from goal. */
    size_t qh = 0, qt = 0;
    pre_queue[qt++] = (Node){ goal, gz0, gz1, 0 };
    pre_insert(goal, 0);

    while (qh < qt) {
        Node cur = pre_queue[qh++];

        if (cur.d >= PRECOMP_MAX_DEPTH) continue;

        /* Generate moves by swapping each empty with adjacent non-empty, excluding other empty. */
        for (int which = 0; which < 2; which++) {
            uint8_t epos = (which == 0) ? cur.z0 : cur.z1;
            uint8_t other = (which == 0) ? cur.z1 : cur.z0;

            for (uint8_t k = 0; k < adj_deg[epos]; k++) {
                uint8_t npos = adj[epos][k];
                if (npos == other) continue; /* never swap empty with empty */
                /* Neighbor must be non-empty by rule; if input violates, behavior unspecified. */
                if (get_nib(cur.s, npos) == 0) continue;

                uint64_t ns = swap_nib(cur.s, epos, npos);

                uint8_t nd = (uint8_t)(cur.d + 1);
                uint8_t existing;
                if (!pre_lookup(ns, &existing)) {
                    pre_insert(ns, nd);

                    uint8_t nz0 = cur.z0, nz1 = cur.z1;
                    if (which == 0) nz0 = npos; else nz1 = npos;

                    pre_queue[qt++] = (Node){ ns, nz0, nz1, nd };
                    /* If qt exceeds MAX_PRECOMP_STATES, behavior is unspecified per spec. */
                }
            }
        }
    }
}

/* Query-side bounded BFS up to 4 moves, looking for any state in precomputed set. */
typedef struct {
    uint64_t s;
    uint8_t z0, z1;
    uint8_t d;
} QNode;

static QNode q_queue[MAX_QUERY_VISITED];

/* Small visited set for query BFS to avoid trivial cycles; fixed-size, unspecified if exceeded. */
#define Q_HASH_SIZE (1u<<20) /* 1,048,576 */
static uint64_t q_keys[Q_HASH_SIZE];
static uint8_t  q_used[Q_HASH_SIZE];

static inline void q_clear(void) {
    /* Clear used flags; O(size). Acceptable given constraints. */
    memset(q_used, 0, sizeof(q_used));
}

static inline int q_mark_if_new(uint64_t key) {
    uint32_t h = hash64(key) & (Q_HASH_SIZE - 1u);
    while (q_used[h]) {
        if (q_keys[h] == key) return 0;
        h = (h + 1u) & (Q_HASH_SIZE - 1u);
    }
    q_used[h] = 1;
    q_keys[h] = key;
    return 1;
}

static int solve_query(uint64_t start) {
    uint8_t dist0;
    if (pre_lookup(start, &dist0)) return (int)dist0;

    uint8_t sz0, sz1;
    find_zeros(start, &sz0, &sz1);

    q_clear();

    size_t qh = 0, qt = 0;
    q_queue[qt++] = (QNode){ start, sz0, sz1, 0 };
    (void)q_mark_if_new(start);

    while (qh < qt) {
        QNode cur = q_queue[qh++];

        if (cur.d >= QUERY_MAX_DEPTH) continue;

        for (int which = 0; which < 2; which++) {
            uint8_t epos = (which == 0) ? cur.z0 : cur.z1;
            uint8_t other = (which == 0) ? cur.z1 : cur.z0;

            for (uint8_t k = 0; k < adj_deg[epos]; k++) {
                uint8_t npos = adj[epos][k];
                if (npos == other) continue;
                if (get_nib(cur.s, npos) == 0) continue;

                uint64_t ns = swap_nib(cur.s, epos, npos);
                uint8_t nd = (uint8_t)(cur.d + 1);

                uint8_t pre_d;
                if (pre_lookup(ns, &pre_d)) {
                    return (int)nd + (int)pre_d;
                }

                if (nd < QUERY_MAX_DEPTH) {
                    if (q_mark_if_new(ns)) {
                        uint8_t nz0 = cur.z0, nz1 = cur.z1;
                        if (which == 0) nz0 = npos; else nz1 = npos;
                        q_queue[qt++] = (QNode){ ns, nz0, nz1, nd };
                        /* If qt exceeds MAX_QUERY_VISITED, behavior is unspecified per spec. */
                    }
                }
            }
        }
    }

    return -1; /* NA */
}

int main(void) {
    precompute();

    for (;;) {
        int v0;
        if (scanf("%d", &v0) != 1) return 0;

        if (v0 < 0) return 0;

        uint8_t a[13];
        a[0] = (uint8_t)(v0 & 0xFF);

        for (int i = 1; i < 13; i++) {
            int vi;
            if (scanf("%d", &vi) != 1) return 0; /* precondition says 13 ints; otherwise unspecified */
            a[i] = (uint8_t)(vi & 0xFF);
        }

        uint64_t s = pack_state(a);
        int ans = solve_query(s);
        if (ans >= 0) {
            printf("%d\n", ans);
        } else {
            printf("NA\n");
        }
    }
}
