#include <stdio.h>
#include <math.h>

static double predict_value(int d, int n, const double v[], int i_excl, int j_pred) {
    /* Build the unique polynomial of degree <= d through the (d+1) points
       with indices k != i_excl and k != j_pred, then evaluate it at x=j_pred.
       Use Lagrange interpolation directly. */
    int idx[10];
    int m = 0;
    for (int k = 0; k < n; k++) {
        if (k == i_excl || k == j_pred) continue;
        idx[m++] = k;
    }
    /* m should be d+1 given n=d+3 */
    double x = (double)j_pred;
    double y = 0.0;

    for (int a = 0; a < m; a++) {
        double xa = (double)idx[a];
        double term = v[idx[a]];
        for (int b = 0; b < m; b++) {
            if (b == a) continue;
            double xb = (double)idx[b];
            term *= (x - xb) / (xa - xb);
        }
        y += term;
    }
    return y;
}

int main(void) {
    int d;
    while (scanf("%d", &d) == 1) {
        if (d == 0) break;

        int n = d + 3;
        double v[10];
        for (int i = 0; i < n; i++) {
            if (scanf("%lf", &v[i]) != 1) return 0; /* unspecified on malformed input */
        }

        int chosen = 0; /* if none satisfies, spec says output is not defined; keep control-flow value */
        for (int i = 0; i < n; i++) {
            int ok = 1;
            for (int j = 0; j < n; j++) {
                if (j == i) continue;
                double pred = predict_value(d, n, v, i, j);
                if (fabs(pred - v[j]) > 0.1) {
                    ok = 0;
                    break;
                }
            }
            if (ok) {
                chosen = i;
                break; /* smallest i */
            }
        }

        printf("%d\n", chosen);
    }
    return 0;
}
