#include <stdio.h>
#include <math.h>

double segment(double x1, double y1, double x2, double y2);
double inclination(double x1, double y1, double x2, double y2);

int main(void)
{
    int x1, x2, y1, y2, x, y;
    double a, b, c;
    int n, i;

    scanf("%d%d%d%d", &x1, &y1, &x2, &y2);
    a = inclination(x1, y1, x2, y2);
    b = -1.0;
    c = segment(x1, y1, x2, y2);
    //ax+by+c=0
    scanf("%d",&n);

    if (a > 2000000000) {
        for (i = 0; i < n; i++) {
            scanf("%d %d", &x, &y);
            if (x < x1) {
                printf("%.10lf %.10lf\n",x+2.0*(x1-x), (double)y);
            } else {
                printf("%.10lf %.10lf\n",x-2.0*(x-x1), (double)y);
            }
        }
    } else {
        for (i = 0; i < n; i++) {
            scanf("%d %d", &x, &y);
            printf("%.10lf %.10lf\n",x-2.0*a*(a*x+b*y+c)/(a*a+b*b)
                    ,y-2.0*b*(a*x+b*y+c)/(a*a+b*b));
        }
    }
    return 0;
}

double segment(double x1, double y1, double x2, double y2)
{
    //*切片
    double a, b1, b2;
    a = inclination(x1, y1, x2, y2);
    if (a > 2000000000.0) {
        return 2100000000.0;
    }
    b1 = -a * x1 + y1;
    b2 = -a * x2 + y2;
    return (b1 + b2) / 2;
}

double inclination(double x1, double y1, double x2, double y2) 
{
    //*傾き
    if (-0.00000000002 < x2 - x1 && x2 - x1 < 0.00000000002) {
        return 2100000000.0;
    } else {
        return (y2 - y1) / (x2 - x1);
    }
}