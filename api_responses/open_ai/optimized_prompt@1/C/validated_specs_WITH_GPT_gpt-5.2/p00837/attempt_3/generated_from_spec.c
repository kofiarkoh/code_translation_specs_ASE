#include <stdio.h>
#include <string.h>

#define MAX_M 11          /* desks 1..m plus holding desk m+1; m is assumed small */
#define MAX_BOOK 101      /* book ids are positive and <= 100 */
#define MAX_REQ 6000      /* total request occurrences bounded (~5000) */

typedef struct {
    int seq;
    int stu;
    int id;
} Req;

static int desk[MAX_M + 2][MAX_REQ]; /* desk index up to m+1; each holds a sequence of book ids */
static int dsz[MAX_M + 2];           /* current sizes */
static int locDesk[MAX_BOOK];        /* current desk holding book id (0 means unseen) */
static int locPos[MAX_BOOK];         /* position within that desk */

static Req reqs[MAX_REQ];
static int reqCount;

static void remove_at(int d, int pos) {
    /* remove desk[d][pos], shift left, and update locPos for shifted books */
    int i;
    for (i = pos; i + 1 < dsz[d]; i++) {
        int moved = desk[d][i + 1];
        desk[d][i] = moved;
        locPos[moved] = i;
    }
    dsz[d]--;
}

static void insert_back(int d, int id) {
    desk[d][dsz[d]] = id;
    locDesk[id] = d;
    locPos[id] = dsz[d];
    dsz[d]++;
}

static void insert_at(int d, int pos, int id) {
    /* insert at position pos (0..dsz[d]), shift right, update locPos */
    int i;
    for (i = dsz[d]; i > pos; i--) {
        int moved = desk[d][i - 1];
        desk[d][i] = moved;
        locPos[moved] = i;
    }
    desk[d][pos] = id;
    locDesk[id] = d;
    locPos[id] = pos;
    dsz[d]++;
}

static int cmp_req(const void *a, const void *b) {
    const Req *ra = (const Req *)a;
    const Req *rb = (const Req *)b;
    if (ra->seq != rb->seq) return (ra->seq < rb->seq) ? -1 : 1;
    if (ra->stu != rb->stu) return (ra->stu < rb->stu) ? -1 : 1;
    return 0;
}

int main(void) {
    int m;
    while (scanf("%d", &m) == 1) {
        if (m <= 0) break;

        int c, n;
        if (scanf("%d %d", &c, &n) != 2) return 0;

        /* reset state */
        memset(dsz, 0, sizeof(dsz));
        memset(locDesk, 0, sizeof(locDesk));
        memset(locPos, 0, sizeof(locPos));
        reqCount = 0;

        int holding = m + 1;

        /* read students and introduce books on first sight into holding desk */
        for (int stu = 0; stu < n; stu++) {
            int t;
            scanf("%d", &t);
            for (int seq = 0; seq < t; seq++) {
                int id;
                scanf("%d", &id);

                if (locDesk[id] == 0) {
                    /* first time seen: place at back of holding desk (not capacity-limited) */
                    desk[holding][dsz[holding]] = id;
                    locDesk[id] = holding;
                    locPos[id] = dsz[holding];
                    dsz[holding]++;
                }

                /* collect request occurrence */
                reqs[reqCount].seq = seq;
                reqs[reqCount].stu = stu;
                reqs[reqCount].id  = id;
                reqCount++;
            }
        }

        /* sort requests by (seq, stu) */
        qsort(reqs, (size_t)reqCount, sizeof(reqs[0]), cmp_req);

        long long cost = 0;

        for (int r = 0; r < reqCount; r++) {
            int id = reqs[r].id;

            /* A) remove requested book from its current desk */
            int from = locDesk[id];
            int pos  = locPos[id];
            remove_at(from, pos);
            cost += from;

            /* B) overflow procedure if primary desk is full before inserting id */
            if (dsz[1] == c) {
                /* 1) choose smallest non-full auxiliary desk tmp in 2..m */
                int tmp = 2;
                while (tmp <= m && dsz[tmp] == c) tmp++;
                /* precondition: tmp must be found */
                insert_back(tmp, id);
                cost += tmp;

                /* 2) evict front book from primary desk */
                int id2 = desk[1][0];
                remove_at(1, 0);
                cost += 1;

                /* 3) choose smallest non-full auxiliary desk j in 2..m */
                int j = 2;
                while (j <= m && dsz[j] == c) j++;
                /* precondition: j must be found */

                /* insert id2: normally at back; if j==tmp, one slot earlier */
                if (j == tmp) {
                    /* cancel the temporary insertion of id by overwriting its slot in ordering */
                    int insertPos = dsz[j] - 1; /* after step (1), id is at last position */
                    /* remove the temporary id from tmp by shifting left over it */
                    /* Since id is at position dsz[j]-1, removing it is just decrement size. */
                    dsz[j]--; /* drop id */
                    /* now insert id2 at the back (which is the former slot) */
                    insert_back(j, id2);
                } else {
                    insert_back(j, id2);
                }
                cost += (long long)j + (long long)tmp;

                /* after this, id is no longer in tmp; it will be inserted into primary below */
                /* ensure id location is not left pointing to tmp */
                locDesk[id] = 0;
                locPos[id] = 0;
            }

            /* C) insert requested book into primary desk */
            insert_back(1, id);
            cost += 1;
        }

        printf("%lld\n", cost);
    }
    return 0;
}
