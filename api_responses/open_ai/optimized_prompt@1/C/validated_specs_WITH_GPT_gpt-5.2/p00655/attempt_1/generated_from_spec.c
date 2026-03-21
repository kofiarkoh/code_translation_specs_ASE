#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
  Translation notes / narrow interpretation choices:

  - The specification defines a dynamic sequence partitioned into two halves with size constraints.
    It also defines removal as "the most recently added element among those currently in the first half"
    (i.e., stack-like by insertion time, but restricted to elements that are in the first half at removal time).

  - The specification does NOT define how elements are assigned to halves on insertion, nor how rebalancing
    chooses which element to move between halves. It only states that rebalancing may move one element
    from second to first to restore balance.

  - To preserve the "most recently added in first half" removal rule, we model halves as *time-ordered*
    partitions of the insertion sequence:
      * first half = earliest ceil(n/2) inserted elements currently present
      * second half = latest floor(n/2) inserted elements currently present
    This makes the "top of first half" the most recently added among the first half, i.e., the boundary element.

  - Rebalancing is then deterministic:
      * after insertion, if second becomes larger than first, move the oldest element of second to the end of first
      * after removal, if first becomes smaller than second, move the oldest element of second to the end of first
    This matches "possibly moving one element from the second half to the first half".

  - Order-statistic queries are by VALUE within a half (multiset semantics, duplicates allowed), independent of time.
    We implement k-th smallest/largest by maintaining per-half order-statistic treaps keyed by (value, unique_id).

  - Input operation codes are not specified. We accept the following minimal, explicit set:
      'I x' : insert value x
      'R'   : remove (prints removed value)
      'A k' : k-th smallest in first half
      'B k' : k-th largest  in first half
      'C k' : k-th smallest in second half
      'D k' : k-th largest  in second half
    Any other code => undefined behavior per spec (we do not attempt recovery).

  - Capacity: fixed upper limit inferred; we set MAXN to 200000 by default. Exceeding it is undefined.
*/

#define MAXN 200000

typedef struct Node {
    int val;
    int id;
    unsigned pr;
    int sz;
    struct Node *l, *r;
} Node;

static Node pool[MAXN + 5];
static int pool_used;

static unsigned rng_state = 2463534242u;
static unsigned rng_u32(void) {
    /* xorshift32 */
    unsigned x = rng_state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    rng_state = x;
    return x;
}

static int nsz(Node *t) { return t ? t->sz : 0; }
static void pull(Node *t) { if (t) t->sz = 1 + nsz(t->l) + nsz(t->r); }

static int key_less(int v1, int id1, int v2, int id2) {
    if (v1 != v2) return v1 < v2;
    return id1 < id2;
}

static Node* new_node(int val, int id) {
    Node *t = &pool[pool_used++];
    t->val = val;
    t->id = id;
    t->pr = rng_u32();
    t->sz = 1;
    t->l = t->r = NULL;
    return t;
}

static void split(Node *t, int val, int id, Node **a, Node **b) {
    /* split into a: keys < (val,id), b: keys >= (val,id) */
    if (!t) { *a = *b = NULL; return; }
    if (key_less(t->val, t->id, val, id)) {
        split(t->r, val, id, &t->r, b);
        *a = t;
        pull(*a);
    } else {
        split(t->l, val, id, a, &t->l);
        *b = t;
        pull(*b);
    }
}

static Node* merge(Node *a, Node *b) {
    if (!a) return b;
    if (!b) return a;
    if (a->pr < b->pr) {
        a->r = merge(a->r, b);
        pull(a);
        return a;
    } else {
        b->l = merge(a, b->l);
        pull(b);
        return b;
    }
}

static Node* insert_node(Node *t, Node *it) {
    if (!t) return it;
    if (it->pr < t->pr) {
        split(t, it->val, it->id, &it->l, &it->r);
        pull(it);
        return it;
    }
    if (key_less(it->val, it->id, t->val, t->id)) t->l = insert_node(t->l, it);
    else t->r = insert_node(t->r, it);
    pull(t);
    return t;
}

static Node* erase_key(Node *t, int val, int id) {
    if (!t) return NULL;
    if (t->val == val && t->id == id) {
        Node *m = merge(t->l, t->r);
        return m;
    }
    if (key_less(val, id, t->val, t->id)) t->l = erase_key(t->l, val, id);
    else t->r = erase_key(t->r, val, id);
    pull(t);
    return t;
}

static Node* kth(Node *t, int k) {
    /* 1-indexed */
    int ls = nsz(t->l);
    if (k <= ls) return kth(t->l, k);
    if (k == ls + 1) return t;
    return kth(t->r, k - ls - 1);
}

/* Time-ordered halves implemented as two deques of element IDs. */
typedef struct {
    int *buf;
    int cap;
    int head, tail; /* [head, tail) */
    int sz;
} Deque;

static void dq_init(Deque *d, int cap, int *storage) {
    d->buf = storage;
    d->cap = cap;
    d->head = d->tail = 0;
    d->sz = 0;
}
static void dq_clear(Deque *d) { d->head = d->tail = d->sz = 0; }
static void dq_push_back(Deque *d, int x) {
    d->buf[d->tail] = x;
    d->tail = (d->tail + 1) % d->cap;
    d->sz++;
}
static int dq_pop_back(Deque *d) {
    d->tail = (d->tail - 1 + d->cap) % d->cap;
    int x = d->buf[d->tail];
    d->sz--;
    return x;
}
static void dq_push_front(Deque *d, int x) {
    d->head = (d->head - 1 + d->cap) % d->cap;
    d->buf[d->head] = x;
    d->sz++;
}
static int dq_pop_front(Deque *d) {
    int x = d->buf[d->head];
    d->head = (d->head + 1) % d->cap;
    d->sz--;
    return x;
}

static int id_val[MAXN + 5];

static void rebalance(Deque *first, Deque *second, Node **tFirst, Node **tSecond) {
    /* Ensure: size(first) >= size(second) and diff <= 1 */
    while (first->sz < second->sz) {
        /* move oldest of second to end of first */
        int id = dq_pop_front(second);
        *tSecond = erase_key(*tSecond, id_val[id], id);
        dq_push_back(first, id);
        *tFirst = insert_node(*tFirst, new_node(id_val[id], id));
    }
    while (first->sz > second->sz + 1) {
        /* move newest of first to front of second */
        int id = dq_pop_back(first);
        *tFirst = erase_key(*tFirst, id_val[id], id);
        dq_push_front(second, id);
        *tSecond = insert_node(*tSecond, new_node(id_val[id], id));
    }
}

int main(void) {
    int m;
    static int storage1[MAXN + 5];
    static int storage2[MAXN + 5];
    Deque first, second;

    while (scanf("%d", &m) == 1) {
        if (m == 0) break;

        pool_used = 0;
        dq_init(&first, MAXN + 5, storage1);
        dq_init(&second, MAXN + 5, storage2);
        dq_clear(&first);
        dq_clear(&second);

        Node *tFirst = NULL, *tSecond = NULL;

        int next_id = 1;

        for (int i = 0; i < m; i++) {
            char op[8];
            if (scanf("%7s", op) != 1) return 0;

            if (op[0] == 'I') {
                int x;
                scanf("%d", &x);
                int id = next_id++;
                id_val[id] = x;

                /* Insert at end of overall sequence => goes to end of second half */
                dq_push_back(&second, id);
                tSecond = insert_node(tSecond, new_node(x, id));

                rebalance(&first, &second, &tFirst, &tSecond);
            } else if (op[0] == 'R') {
                /* Remove "top" of first half: most recent in first => back of first deque */
                int id = dq_pop_back(&first);
                int x = id_val[id];
                tFirst = erase_key(tFirst, x, id);
                printf("%d\n", x);

                rebalance(&first, &second, &tFirst, &tSecond);
            } else if (op[0] == 'A') {
                int k; scanf("%d", &k);
                Node *res = kth(tFirst, k);
                printf("%d\n", res->val);
            } else if (op[0] == 'B') {
                int k; scanf("%d", &k);
                int sz = nsz(tFirst);
                Node *res = kth(tFirst, sz - k + 1);
                printf("%d\n", res->val);
            } else if (op[0] == 'C') {
                int k; scanf("%d", &k);
                Node *res = kth(tSecond, k);
                printf("%d\n", res->val);
            } else if (op[0] == 'D') {
                int k; scanf("%d", &k);
                int sz = nsz(tSecond);
                Node *res = kth(tSecond, sz - k + 1);
                printf("%d\n", res->val);
            } else {
                /* Undefined behavior per spec for malformed/unknown op; do minimal. */
                /* Attempt to continue by ignoring line is not specified; we just do nothing. */
            }
        }

        printf("end\n");
    }

    return 0;
}
