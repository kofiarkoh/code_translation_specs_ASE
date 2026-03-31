#include <stdio.h>

#define MAXN 102
#define MAXPAIRS 51
#define QCAP 102

typedef struct {
    int use[MAXPAIRS];
    int chg[MAXPAIRS];
    int m;              /* number of pairs */
    int idx;            /* current pair index */
    int state;          /* 0 = using, 1 = charging */
    int rem;            /* remaining time in current state segment */
    int queued;         /* 1 if currently in charging queue, else 0 */
} Battery;

typedef struct {
    int a[QCAP];
    int head, tail, size;
} Queue;

static void q_init(Queue *q) {
    q->head = q->tail = q->size = 0;
}

static void q_push(Queue *q, int v) {
    /* Behavior is unspecified on overflow per spec; we do not guard. */
    q->a[q->tail] = v;
    q->tail = (q->tail + 1) % QCAP;
    q->size++;
}

static int q_pop(Queue *q) {
    int v = q->a[q->head];
    q->head = (q->head + 1) % QCAP;
    q->size--;
    return v;
}

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n <= 0) break;

        int lim;
        if (scanf("%d", &lim) != 1) break;

        Battery b[MAXN];
        for (int i = 0; i < n; i++) {
            b[i].m = 0;
            b[i].idx = 0;
            b[i].state = 0; /* using */
            b[i].queued = 0;

            for (;;) {
                int u;
                if (scanf("%d", &u) != 1) return 0;
                if (u <= 0) break;
                int c;
                if (scanf("%d", &c) != 1) return 0;

                if (b[i].m < MAXPAIRS) {
                    b[i].use[b[i].m] = u;
                    b[i].chg[b[i].m] = c;
                    b[i].m++;
                } else {
                    /* Unspecified if too many pairs; still consume input and ignore extras. */
                }
            }

            /* Preconditions say at least one pair exists; otherwise unspecified. */
            b[i].rem = (b[i].m > 0) ? b[i].use[0] : 0;
        }

        Queue q;
        q_init(&q);

        long long missed = 0;

        for (int t = 0; t < lim; t++) {
            /* 1) Using progresses for all using batteries; those that finish enter charging and queue. */
            for (int i = 0; i < n; i++) {
                if (b[i].state == 0) {
                    b[i].rem -= 1;
                    if (b[i].rem == 0) {
                        b[i].state = 1; /* charging */
                        b[i].rem = b[i].chg[b[i].idx];
                        if (!b[i].queued) {
                            q_push(&q, i);
                            b[i].queued = 1;
                        }
                    }
                }
            }

            /* 2) Select at most one queued battery to receive one unit of charging progress. */
            int selected = -1;
            if (q.size > 0) {
                selected = q_pop(&q);
                b[selected].queued = 0;
            }

            /* 3) Count missed opportunities: charging but not selected. */
            for (int i = 0; i < n; i++) {
                if (b[i].state == 1 && i != selected) {
                    missed += 1;
                }
            }

            /* 4) Apply charging progress to selected battery (if any). */
            if (selected != -1) {
                b[selected].rem -= 1;
                if (b[selected].rem == 0) {
                    /* Charging complete: advance cycle element and start using next use duration. */
                    if (b[selected].m > 0) {
                        b[selected].idx = (b[selected].idx + 1) % b[selected].m;
                        b[selected].state = 0; /* using */
                        b[selected].rem = b[selected].use[b[selected].idx];
                    } else {
                        /* Unspecified if no pairs; keep in using with rem=0. */
                        b[selected].state = 0;
                        b[selected].rem = 0;
                    }
                } else {
                    /* Still charging: re-enqueue to preserve FIFO among those waiting. */
                    if (!b[selected].queued) {
                        q_push(&q, selected);
                        b[selected].queued = 1;
                    }
                }
            }
        }

        printf("%lld\n", missed);
    }

    return 0;
}
