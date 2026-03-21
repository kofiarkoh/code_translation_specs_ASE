#include <stdio.h>
#include <math.h>

#define MAXN 10
#define TOL 0.1

/* Evaluate the unique degree-(m-1) polynomial passing through points
   (xs[k], ys[k]) for k=0..m-1 at xq using Lagrange interpolation. */
static double lagrange_eval(const double *xs, const double *ys, int m, double xq) {
    double sum = 0.0;
    for (int i = 0; i < m; i++) {
        double term = ys[i];
        for (int j = 0; j < m; j++) {
            if (j == i) continue;
            term *= (xq - xs[j]) / (xs[i] - xs[j]);
        }
        sum += term;
    }
    return sum;
}

/* For fixed i and j (i != j), build the degree-d interpolant from all points
   except i and j (so exactly d+1 points remain), and predict y at index j. */
static double predict_excluding_ij(const double *y, int n, int i, int j) {
    double xs[MAXN], ys[MAXN];
    int m = 0;
    for (int k = 0; k < n; k++) {
        if (k == i || k == j) continue;
        xs[m] = (double)k;
        ys[m] = y[k];
        m++;
    }
    /* By construction in this problem, m == d+1 and defines a unique degree-d polynomial. */
    return lagrange_eval(xs, ys, m, (double)j);
}

int main(void) {
    int d;
    while (scanf("%d", &d) == 1) {
        if (d == 0) break;

        int n = d + 3;
        double y[MAXN];
        for (int k = 0; k < n; k++) {
            if (scanf("%lf", &y[k]) != 1) return 0; /* Undefined per spec; just stop. */
        }

        int outlier = 0; /* If assumption fails, output may be meaningless; keep deterministic. */

        for (int i = 0; i < n; i++) {
            int i_is_outlier = 0;

            /* Check existence of some j != i that is inconsistent (> TOL) when predicted
               from the polynomial built from all points except i and j. */
            for (int j = 0; j < n; j++) {
                if (j == i) continue;
                double pred = predict_excluding_ij(y, n, i, j);
                if (fabs(y[j] - pred) > TOL) {
                    i_is_outlier = 1;
                    break;
                }
            }

            if (i_is_outlier) {
                outlier = i;
                break; /* smallest such index */
            }
        }

        printf("%d\n", outlier);
    }
    return 0;
}
