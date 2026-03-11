#include <stdio.h>
#include <math.h>
#include <stdlib.h>

int main()
{
    int c1x, c1y, c1r, c2x, c2y, c2r;
    double d; //distance of Between the centers

    //printf("input\n");
    scanf("%d %d %d", &c1x, &c1y, &c1r);
    scanf("%d %d %d", &c2x, &c2y, &c2r);

    d = sqrt((c1x-c2x)*(c1x-c2x) + (c1y-c2y)*(c1y-c2y));

    // 4 common tangent lines
    if(d > (c1r+c2r)) printf("4\n");

    // 3 common tangent lines
    if(d == (c1r+c2r)) printf("3\n");

    // 2 common tangent lines
    if((abs(c1r-c2r) < d) && (d < (c1r+c2r))) printf("2\n");

    // 1 common tangent lines
    if(abs(c1r-c2r) == d) printf("1\n");

    // 0 common tangent lines
    if(d < abs(c1r-c2r)) printf("0\n");

    return 0;
}
