#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Node {
    int id;
    int count;
    struct Node *prev;
    struct Node *next;
} Node;

int main(void) {
    int n, m, q;
    if (scanf("%d %d %d", &n, &m, &q) != 3) {
        return 0;
    }

    int *a = NULL;
    if (q > 0) {
        a = (int *)malloc((size_t)q * sizeof(int));
        if (!a) return 0;
        for (int i = 0; i < q; i++) {
            (void)scanf("%d", &a[i]);
        }
    }

    Node *nodes = (Node *)malloc((size_t)(m + 1) * sizeof(Node));
    if (!nodes) {
        free(a);
        return 0;
    }

    for (int i = 0; i <= m; i++) {
        nodes[i].id = i;
        nodes[i].count = 0;
        nodes[i].prev = NULL;
        nodes[i].next = NULL;
    }
    nodes[0].count = n;

    /* Build list by scanning from end; tail pointer p starts at node 0. */
    Node *p = &nodes[0];
    for (int idx = q - 1; idx >= 0; idx--) {
        int x = a[idx];

        if (nodes[x].count == 0) {
            nodes[x].prev = p;
            p->next = &nodes[x];
            p = &nodes[x];
        }

        /* count[x] = min(count[x] + 1, count[prev(x)]) */
        {
            int inc = nodes[x].count + 1;
            int cap = nodes[x].prev ? nodes[x].prev->count : 0;
            nodes[x].count = (inc < cap) ? inc : cap;
        }
    }

    bool *complete = (bool *)calloc((size_t)(m + 1), sizeof(bool));
    if (!complete) {
        free(nodes);
        free(a);
        return 0;
    }

    int allow = 0;
    Node *cur = &nodes[0];

    /* Phase 1: automatic prefix completion while count == n */
    while (cur != NULL && cur->count == n) {
        complete[cur->id] = true;
        while (allow <= m && complete[allow]) allow++;
        cur = cur->next;
    }

    /* Phase 2: ordered completion check */
    while (cur != NULL) {
        if (cur->id != allow) {
            printf("No\n");
            free(complete);
            free(nodes);
            free(a);
            return 0;
        }
        complete[cur->id] = true;
        while (allow <= m && complete[allow]) allow++;
        cur = cur->next;
    }

    printf("Yes\n");

    free(complete);
    free(nodes);
    free(a);
    return 0;
}
