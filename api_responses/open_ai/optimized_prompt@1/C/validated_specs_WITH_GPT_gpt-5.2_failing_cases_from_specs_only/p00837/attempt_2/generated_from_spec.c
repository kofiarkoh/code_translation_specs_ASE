#include <stdio.h>
#include <stdlib.h>

#define MAX_M 10
#define MAX_BOOK_ID 100
#define MAX_REQ 5002
#define MAX_DESKS (MAX_M + 2) /* 1..m plus special m+1 */

typedef struct {
    int seq;     /* position within student's list (0-based) */
    int student; /* student index (0..n-1) */
    int id;      /* book identifier */
} Request;

static int cmp_request(const void *a, const void *b) {
    const Request *ra = (const Request *)a;
    const Request *rb = (const Request *)b;
    if (ra->seq != rb->seq) return (ra->seq < rb->seq) ? -1 : 1;
    if (ra->student != rb->student) return (ra->student < rb->student) ? -1 : 1;
    return 0;
}

int main(void) {
    int m, c, n;

    while (scanf("%d", &m) == 1) {
        if (m <= 0) return 0;
        if (scanf("%d %d", &c, &n) != 2) return 0;

        /* Desk storage: ordered sequences with compaction on removal.
           Capacity c can be 0..5000; total distinct books <= 100 by spec. */
        int desk[MAX_DESKS][MAX_BOOK_ID + 5];
        int dsz[MAX_DESKS];
        for (int i = 0; i < MAX_DESKS; i++) dsz[i] = 0;

        /* Track current desk location for each book id (1..100). 0 means unseen. */
        int loc[MAX_BOOK_ID + 1];
        for (int id = 0; id <= MAX_BOOK_ID; id++) loc[id] = 0;

        Request reqs[MAX_REQ];
        int reqCount = 0;

        int special = m + 1;

        for (int student = 0; student < n; student++) {
            int t;
            scanf("%d", &t);
            for (int k = 0; k < t; k++) {
                int id;
                scanf("%d", &id);

                /* Record request */
                if (reqCount < MAX_REQ) {
                    reqs[reqCount].seq = k;
                    reqs[reqCount].student = student;
                    reqs[reqCount].id = id;
                    reqCount++;
                }

                /* Introduce distinct book on first appearance: place on desk m+1, no cost */
                if (loc[id] == 0) {
                    desk[special][dsz[special]++] = id;
                    loc[id] = special;
                }
            }
        }

        qsort(reqs, (size_t)reqCount, sizeof(reqs[0]), cmp_request);

        long long total = 0;

        for (int r = 0; r < reqCount; r++) {
            int id = reqs[r].id;

            /* 8.1 Remove requested book from its current desk */
            int from = loc[id];
            /* Find and remove id from desk[from], compacting */
            int pos = -1;
            for (int i = 0; i < dsz[from]; i++) {
                if (desk[from][i] == id) { pos = i; break; }
            }
            if (pos >= 0) {
                for (int i = pos + 1; i < dsz[from]; i++) desk[from][i - 1] = desk[from][i];
                dsz[from]--;
            }
            total += from;

            /* 8.2 If primary desk is full before placing id */
            if (dsz[1] == c) {
                /* Step 1: choose tmp in 2..m smallest not full */
                int tmp = -1;
                for (int d = 2; d <= m; d++) {
                    if (dsz[d] < c) { tmp = d; break; }
                }

                /* Step 2: append id to tmp, cost tmp */
                desk[tmp][dsz[tmp]++] = id;
                total += tmp;

                /* Step 3: evict front book from desk 1, cost 1 */
                int id2 = 0;
                if (dsz[1] > 0) {
                    id2 = desk[1][0];
                    for (int i = 1; i < dsz[1]; i++) desk[1][i - 1] = desk[1][i];
                    dsz[1]--;
                }
                total += 1;

                /* Step 4: choose j in 2..m smallest not full after updates */
                int j = -1;
                for (int d = 2; d <= m; d++) {
                    if (dsz[d] < c) { j = d; break; }
                }

                /* Step 5: insert id2 into desk j at pos:
                   pos = end, except if j==tmp then before the temporarily placed id */
                int insertPos = dsz[j];
                if (j == tmp) insertPos = dsz[j] - 1; /* immediately before temp id */
                for (int i = dsz[j]; i > insertPos; i--) desk[j][i] = desk[j][i - 1];
                desk[j][insertPos] = id2;
                dsz[j]++;

                /* Update location of evicted book */
                loc[id2] = j;

                /* Step 6: add j+tmp, then remove temporarily placed id from tmp by size-- */
                total += (j + tmp);
                dsz[tmp]--; /* removes the last element, which is the temporary id */
            }

            /* 8.3 Place requested book onto primary desk, cost 1 */
            desk[1][dsz[1]++] = id;
            total += 1;
            loc[id] = 1;
        }

        printf("%lld\n", total);
    }

    return 0;
}
