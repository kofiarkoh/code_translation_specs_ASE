#include <stdio.h>
#include <math.h>

#define EPS (1e-5)

typedef struct {
    double x;
    double y;
} point_t;


int main() {
    point_t seal[100], *si, *sj;
    point_t p[10000];
    int n, i, j, q, maximum;

    scanf("%d", &n);
    for (i = 0; i < n; i++) {
        scanf("%lf,%lf", &(seal[i].x), &(seal[i].y));
    }

    q = 0;
    for (i = 0; i < n-1; i++) {
        si = &seal[i];
        for (j = i + 1; j < n; j++) {
            double d, to, tp;
            sj = &seal[j];
            d = hypot(sj->x - si->x, sj->y - si->y);
            
            if (d - 2.0 > EPS) continue;
            
            to = atan2(sj->y - si->y, sj->x - si->x);
            tp = acos(d * .5);

            p[q].x = si->x + cos(to + tp);
            p[q].y = si->y + sin(to + tp);
            q++;
            p[q].x = si->x + cos(to - tp);
            p[q].y = si->y + cos(to - tp);
            q++;
        }
    }

    for (i = 0, maximum = 0; i < q; i++) {
        int count;
        count = 0;
        for (j = 0; j < n; j++) {
            if (hypot(seal[j].x - p[i].x, seal[j].y - p[i].y) - 1 < EPS)
                count++;
        }
        maximum = count > maximum ?  count : maximum;
    }
    
    printf("%d\n", maximum);

    return 0;
}