#include <stdio.h>
#include <stdlib.h>

#define MAXN 100002

typedef struct {
    int w;
    int v;
    double d; /* value density v / w */
} Item;

static int cmp_item_desc_density(const void *a, const void *b) {
    const Item *ia = (const Item *)a;
    const Item *ib = (const Item *)b;
    /* Sort by descending density. */
    if (ia->d > ib->d) return -1;
    if (ia->d < ib->d) return 1;
    return 0;
}

int main(void) {
    int n;
    double cap;

    if (scanf("%d", &n) != 1) return 0;
    if (scanf("%lf", &cap) != 1) return 0;

    /* Fixed internal storage limit per spec; if violated, behavior is undefined.
       We still allocate fixed-size storage and proceed. */
    static Item items[MAXN];
    int m = 0;

    double total = 0.0;
    double eff_cap = cap;

    for (int i = 0; i < n; i++) {
        int w, v;
        if (scanf("%d %d", &w, &v) != 2) return 0;

        if (w < 0 && v > 0) {
            /* Fully include: increases total value; negative weight increases effective capacity. */
            total += (double)v;
            eff_cap -= (double)w; /* subtracting negative adds capacity */
        } else if (w >= 0 && v <= 0) {
            /* Ignored per spec. */
        } else {
            /* Candidate for fractional optimization.
               Preconditions state any such item has non-zero effective weight. */
            items[m].w = w;
            items[m].v = v;
            items[m].d = (double)v / (double)w;
            m++;
        }
    }

    qsort(items, (size_t)m, sizeof(items[0]), cmp_item_desc_density);

    /* Greedy fractional knapsack on remaining candidates. */
    const double EPS = 1e-12;
    for (int i = 0; i < m; i++) {
        if (eff_cap <= EPS) break;

        double w = (double)items[i].w;
        double v = (double)items[i].v;

        if (w <= eff_cap) {
            /* Take whole item. */
            total += v;
            eff_cap -= w;
        } else {
            /* Take fraction. */
            double frac = eff_cap / w; /* precondition implies w != 0 */
            if (frac < 0.0) frac = 0.0; /* enforce invariant bounds narrowly */
            if (frac > 1.0) frac = 1.0;
            total += v * frac;
            eff_cap = 0.0;
            break;
        }
    }

    /* Output a single real number. */
    printf("%.10f\n", total);
    return 0;
}
