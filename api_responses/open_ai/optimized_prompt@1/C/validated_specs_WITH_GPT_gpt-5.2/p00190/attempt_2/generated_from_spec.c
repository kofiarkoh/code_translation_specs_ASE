#include <stdio.h>
#include <stdint.h>
#include <string.h>

/*
  13-position puzzle with exactly two empty spaces (0 values).
  Goal: [1..11,0,0] at positions 0..12.

  Behavior per spec:
  - Precompute all states within 16 moves of goal with exact distances.
  - For each query:
      * if in precomputed set: output exact distance (<=16, hence <=20).
      * else BFS outward from query up to depth 4; if any reached state is in
        precomputed set, output depth + precomputed_distance, which is exact
        minimum if <=20 (per spec's method).
      * else output "NA".
  - If minimum > 20, output "NA" (method will not find within bounds).
  - Input ends when first integer of a test case is negative; no output for it.

  Unspecified behavior for invalid inputs (not exactly two zeros, values out of
  intended range, etc.) is not guarded against beyond basic parsing.
*/

/* Adjacency list as given (undirected). */
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
static const uint8_t deg[13] = {1,2,4,2,1,4,4,4,1,2,4,2,1};

#define PRE_DEPTH 16
#define Q_DEPTH 4

/*
  State encoding:
  - 13 positions, each value intended 0..15 (4 bits).
  - Pack into 64-bit: nibble i at bits (4*i).
  This preserves exact tile identities and positions.
*/
static inline uint64_t pack_state(const uint8_t a[13]) {
    uint64_t x = 0;
    for (int i = 0; i < 13; i++) {
        x |= ((uint64_t)(a[i] & 0x0F)) << (4 * i);
    }
    return x;
}
static inline uint8_t get_nib(uint64_t x, int i) {
    return (uint8_t)((x >> (4 * i)) & 0x0FULL);
}
static inline uint64_t set_nib(uint64_t x, int i, uint8_t v) {
    uint64_t mask = (uint64_t)0x0FULL << (4 * i);
    x = (x & ~mask) | (((uint64_t)(v & 0x0F)) << (4 * i));
    return x;
}
static inline uint64_t swap_pos(uint64_t x, int i, int j) {
    uint8_t vi = get_nib(x, i);
    uint8_t vj = get_nib(x, j);
    x = set_nib(x, i, vj);
    x = set_nib(x, j, vi);
    return x;
}

/* Find the two zero positions; behavior unspecified if not exactly two zeros. */
static inline void find_zeros(uint64_t x, uint8_t *z0, uint8_t *z1) {
    uint8_t a = 255, b = 255;
    for (uint8_t i = 0; i < 13; i++) {
        if (get_nib(x, i) == 0) {
            if (a == 255) a = i;
            else { b = i; break; }
        }
    }
    *z0 = a;
    *z1 = b;
}

/* Fixed-size hash table for precomputed distances (open addressing). */
#define HT_SIZE (1u << 22) /* 4,194,304 slots */
#define HT_MASK (HT_SIZE - 1u)

typedef struct {
    uint64_t key;
    uint8_t dist;   /* 0..16 */
    uint8_t used;   /* 0 empty, 1 used */
} HTEntry;

static HTEntry ht[HT_SIZE];

static inline uint32_t hash64(uint64_t x) {
    /* Splitmix64 finalizer-like mix, then take low bits. */
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccdULL;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53ULL;
    x ^= x >> 33;
    return (uint32_t)x;
}

static int ht_get(uint64_t key, uint8_t *out_dist) {
    uint32_t idx = hash64(key) & HT_MASK;
    for (uint32_t probe = 0; probe < HT_SIZE; probe++) {
        HTEntry *e = &ht[(idx + probe) & HT_MASK];
        if (!e->used) return 0;
        if (e->key == key) {
            *out_dist = e->dist;
            return 1;
        }
    }
    return 0; /* table full => unspecified per spec */
}

static void ht_put_if_absent(uint64_t key, uint8_t dist) {
    uint32_t idx = hash64(key) & HT_MASK;
    for (uint32_t probe = 0; probe < HT_SIZE; probe++) {
        HTEntry *e = &ht[(idx + probe) & HT_MASK];
        if (!e->used) {
            e->used = 1;
            e->key = key;
            e->dist = dist;
            return;
        }
        if (e->key == key) {
            /* Already present; keep existing (should be minimal by BFS). */
            return;
        }
    }
    /* If we get here, table is full: unspecified behavior per spec. */
}

/* Precompute BFS from goal up to depth 16. */
typedef struct {
    uint64_t state;
    uint8_t z0, z1;
    uint8_t dist;
} Node;

/* Fixed-size queue; if exceeded, behavior unspecified per spec. */
#define PRE_Q_CAP 2000000u
static Node pre_q[PRE_Q_CAP];

static void precompute(void) {
    uint8_t goal_arr[13];
    for (int i = 0; i < 11; i++) goal_arr[i] = (uint8_t)(i + 1);
    goal_arr[11] = 0;
    goal_arr[12] = 0;
    uint64_t goal = pack_state(goal_arr);

    uint8_t gz0, gz1;
    find_zeros(goal, &gz0, &gz1);

    memset(ht, 0, sizeof(ht));

    uint32_t head = 0, tail = 0;
    pre_q[tail++] = (Node){ goal, gz0, gz1, 0 };
    ht_put_if_absent(goal, 0);

    while (head < tail) {
        Node cur = pre_q[head++];

        if (cur.dist >= PRE_DEPTH) continue;

        /* Generate legal moves: swap each zero with adjacent non-zero tile,
           disallow swapping with the other zero. */
        for (int which = 0; which < 2; which++) {
            uint8_t z = (which == 0) ? cur.z0 : cur.z1;
            uint8_t other = (which == 0) ? cur.z1 : cur.z0;

            for (uint8_t k = 0; k < deg[z]; k++) {
                uint8_t nb = adj[z][k];
                if (nb == other) continue; /* never swap empty with empty */
                /* Neighbor must be non-empty by this check; if input violates
                   "exactly two zeros", behavior is unspecified. */
                if (get_nib(cur.state, nb) == 0) continue;

                uint64_t nxt_state = swap_pos(cur.state, z, nb);
                uint8_t nxt_z0 = cur.z0, nxt_z1 = cur.z1;
                if (which == 0) nxt_z0 = nb; else nxt_z1 = nb;

                uint8_t existing;
                if (!ht_get(nxt_state, &existing)) {
                    uint8_t nd = (uint8_t)(cur.dist + 1);
                    ht_put_if_absent(nxt_state, nd);
                    if (tail < PRE_Q_CAP) {
                        pre_q[tail++] = (Node){ nxt_state, nxt_z0, nxt_z1, nd };
                    } else {
                        /* Queue overflow: unspecified per spec. */
                        return;
                    }
                }
            }
        }
    }
}

/* Query BFS up to depth 4, looking for intersection with precomputed set. */
typedef struct {
    uint64_t state;
    uint8_t z0, z1;
    uint8_t dist;
} QNode;

#define Q_CAP 200000u
static QNode q_q[Q_CAP];

/* Small visited set for query BFS to avoid cycles within depth 4. */
#define QV_SIZE (1u << 18) /* 262,144 */
#define QV_MASK (QV_SIZE - 1u)
static uint64_t qv_keys[QV_SIZE];
static uint8_t qv_used[QV_SIZE];

static void qv_clear(void) {
    memset(qv_used, 0, sizeof(qv_used));
}
static int qv_add_if_absent(uint64_t key) {
    uint32_t idx = hash64(key) & QV_MASK;
    for (uint32_t probe = 0; probe < QV_SIZE; probe++) {
        uint32_t p = (idx + probe) & QV_MASK;
        if (!qv_used[p]) {
            qv_used[p] = 1;
            qv_keys[p] = key;
            return 1;
        }
        if (qv_keys[p] == key) return 0;
    }
    return 0; /* full => treat as already present; unspecified anyway */
}

static int solve(uint64_t start, int *out_moves) {
    uint8_t d0;
    if (ht_get(start, &d0)) {
        *out_moves = (int)d0;
        return 1;
    }

    uint8_t sz0, sz1;
    find_zeros(start, &sz0, &sz1);

    qv_clear();
    qv_add_if_absent(start);

    uint32_t head = 0, tail = 0;
    q_q[tail++] = (QNode){ start, sz0, sz1, 0 };

    while (head < tail) {
        QNode cur = q_q[head++];

        if (cur.dist >= Q_DEPTH) continue;

        for (int which = 0; which < 2; which++) {
            uint8_t z = (which == 0) ? cur.z0 : cur.z1;
            uint8_t other = (which == 0) ? cur.z1 : cur.z0;

            for (uint8_t k = 0; k < deg[z]; k++) {
                uint8_t nb = adj[z][k];
                if (nb == other) continue;
                if (get_nib(cur.state, nb) == 0) continue;

                uint64_t nxt_state = swap_pos(cur.state, z, nb);
                uint8_t nxt_z0 = cur.z0, nxt_z1 = cur.z1;
                if (which == 0) nxt_z0 = nb; else nxt_z1 = nb;

                uint8_t nd = (uint8_t)(cur.dist + 1);

                uint8_t pre_d;
                if (ht_get(nxt_state, &pre_d)) {
                    int total = (int)nd + (int)pre_d;
                    if (total <= 20) {
                        *out_moves = total;
                        return 1;
                    }
                    /* If total > 20, spec says output NA; keep searching in case
                       another intersection yields <=20 (though if true minimum
                       >20, none will). */
                }

                if (nd < Q_DEPTH) {
                    if (qv_add_if_absent(nxt_state)) {
                        if (tail < Q_CAP) {
                            q_q[tail++] = (QNode){ nxt_state, nxt_z0, nxt_z1, nd };
                        } else {
                            /* Queue overflow: unspecified per spec. */
                            return 0;
                        }
                    }
                }
            }
        }
    }

    return 0;
}

int main(void) {
    precompute();

    for (;;) {
        int in0;
        if (scanf("%d", &in0) != 1) return 0;
        if (in0 < 0) return 0;

        uint8_t arr[13];
        arr[0] = (uint8_t)in0;
        for (int i = 1; i < 13; i++) {
            int v;
            if (scanf("%d", &v) != 1) return 0; /* precondition violated */
            arr[i] = (uint8_t)v;
        }

        uint64_t start = pack_state(arr);
        int moves;
        if (solve(start, &moves)) {
            printf("%d\n", moves);
        } else {
            printf("NA\n");
        }
    }
}
