#include <stdio.h>
#include <math.h>

int main(void) {
    int t;
    if (scanf("%d", &t) != 1) return 0;

    for (int i = 0; i < t; i++) {
        double x1, y1, x2, y2, x3, y3;
        if (scanf("%lf %lf %lf %lf %lf %lf", &x1, &y1, &x2, &y2, &x3, &y3) != 6) {
            /* Specification: no explicit handling for insufficient input; terminate quietly. */
            return 0;
        }

        /* Circumcenter formula using determinants.
           Preconditions guarantee non-collinearity and distinct points, so denominator != 0. */
        double a = x1 - x2;
        double b = y1 - y2;
        double c = x1 - x3;
        double d = y1 - y3;

        double e = ((x1 * x1 - x2 * x2) + (y1 * y1 - y2 * y2)) / 2.0;
        double f = ((x1 * x1 - x3 * x3) + (y1 * y1 - y3 * y3)) / 2.0;

        double det = a * d - b * c;

        double cx = (e * d - b * f) / det;
        double cy = (a * f - e * c) / det;

        /* Radius is distance from center to first point (precondition: non-zero, finite). */
        double dx = cx - x1;
        double dy = cy - y1;
        double r = sqrt(dx * dx + dy * dy);

        printf("%.3f %.3f %.3f\n", cx, cy, r);
    }

    return 0;
}
