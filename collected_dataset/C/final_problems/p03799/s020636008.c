#include <stdio.h>
#include <math.h>
int main()
{
    double a, b, c;
    scanf("%lf%lf", &a, &b);
    if(b<2*a) c=b/2;
    else
    {
        c=a+(b-2*a)/4;
    }
    printf("%.0lf\n", floor(c));
    return 0;
}
