#include <stdio.h>
#include <math.h>

typedef struct
{
    double x;
    double y;
    double r;
}circle;

int main(void)
{
    int datasets;
    scanf("%d", &datasets);

    while (datasets--) {
        circle a, b;
        scanf(" %lf %lf %lf", &(a.x), &(a.y), &(a.r));
        scanf(" %lf %lf %lf", &(b.x), &(b.y), &(b.r));

        double bigr, smallr;
        if (a.r >=  b.r) {
            bigr = a.r;
            smallr = b.r;
        } else {
            bigr = b.r;
            smallr = a.r;
        }

        double dist = pow(a.x - b.x, 2) + pow(a.y - b.y, 2);
        dist = sqrt(dist);

        int judge;
        if (dist > bigr + smallr) {
            judge = 0;
        } else if (dist > bigr - smallr) {
            judge = 1;
        } else {
            if (bigr == a.r) {
                judge = 2;
            } else {
                judge = -2;
            }
        }

        printf("%d\n", judge);
    }

    return 0;
}