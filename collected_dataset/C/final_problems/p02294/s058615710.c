#include <stdio.h>
#include <math.h>

#define PREC 1e-12

int main() {
    int x, y, x0, y0, x1, y1, x2, y2, x3, y3;
    int q;
    double s, t, denom;
    int judge;

    scanf("%d", &q);
    
    while (q--) {
        scanf("%d %d %d %d %d %d %d %d",
              &x0, &y0, &x1, &y1, &x2, &y2, &x3, &y3);
        denom = (x1-x0)*(y3-y2) - (y1-y0)*(x3-x2);
        if (fabs(denom) < PREC) {
            judge = 0;
        }
        else {
            s = ((y3-y2)*(x2-x0)-(x3-x2)*(y2-y0)) / denom;
            t = ((y1-y0)*(x2-x0)-(x1-x0)*(y2-y0)) / denom;
            if (s < 0 || s > 1 || t < 0 || t > 1) {
                judge = 0;
            }
            else {
                judge = 1;
            }
        }
        printf("%d\n", judge);
    }
}