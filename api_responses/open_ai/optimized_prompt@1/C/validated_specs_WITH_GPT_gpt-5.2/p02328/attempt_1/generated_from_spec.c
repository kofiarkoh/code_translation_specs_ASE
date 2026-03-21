#include <stdio.h>
#include <stdint.h>

#define MAX_W 100001

typedef struct {
    int64_t height;
    int64_t start; /* earliest index where this height can extend */
} StackEntry;

int main(void) {
    int w_in;
    if (scanf("%d", &w_in) != 1) {
        /* Undefined behavior per spec on invalid/missing input; just exit. */
        return 0;
    }

    int64_t w = (int64_t)w_in;

    if (w == 0) {
        printf("0");
        return 0;
    }

    /* Fixed-size storage as per spec; behavior undefined if w > MAX_W. */
    StackEntry st[MAX_W + 1];
    int top = 0;

    int64_t best = 0;

    for (int64_t i = 0; i < w; i++) {
        int64_t h;
        if (scanf("%" SCNd64, &h) != 1) {
            /* Undefined behavior per spec on invalid/missing input; just exit. */
            return 0;
        }

        int64_t start = i;

        /* Maintain increasing heights; close rectangles when current height is smaller. */
        while (top > 0 && st[top - 1].height > h) {
            StackEntry e = st[--top];
            int64_t area = e.height * (i - e.start);
            if (area > best) best = area;
            start = e.start; /* current bar can extend back to where popped bar started */
        }

        /* If equal height exists, keep the earliest start (narrowest interpretation:
           merge by not pushing a duplicate height). */
        if (top > 0 && st[top - 1].height == h) {
            if (start < st[top - 1].start) st[top - 1].start = start;
        } else {
            st[top].height = h;
            st[top].start = start;
            top++;
        }
    }

    /* Close remaining rectangles to the end. */
    for (int j = 0; j < top; j++) {
        int64_t area = st[j].height * (w - st[j].start);
        if (area > best) best = area;
    }

    printf("%" PRId64, best);
    return 0;
}
