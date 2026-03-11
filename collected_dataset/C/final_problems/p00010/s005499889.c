#include <stdio.h>
#include <math.h>

double ceilN(double a, int N){
  double ans,sign;
  sign = (a < 0)? -1 : 1;
  ans = a * pow(10.0,N);
  ans = (double)(int)(ans+ 0.5 * sign);
  return ans * pow(10.0,-N);
}
int main()
{
  int i, n;
  double x1, x2, x3, y1, y2, y3;
  double a, b, c, d, e, f;
  double X1, Y1, r;

  scanf("%d", &n);
  for (i = 0; i < n; i++){
    scanf("%lf %lf %lf %lf %lf %lf",&x1, &y1, &x2, &y2, &x3, &y3);
    a = 2 * (x2 - x1);
    b = 2 * (y2 - y1);
    c = x1*x1 - x2*x2 + y1*y1 - y2*y2;
    d = 2 * (x3 - x1);
    e = 2 * (y3 - y1);
    f = x1*x1 - x3*x3 + y1*y1 - y3*y3;

    X1 = (b*f - e*c) / (a*e - d*b);
    Y1 = (c*d - f*a) / (a*d - d*b);
    r  = sqrt((X1-x1)*(X1-x1) + (Y1-y1)*(Y1-y1));

    X1 = ceilN(X1,3);
    Y1 = ceilN(Y1,3);
    r = ceilN(r,3);

    printf("%.3f %.3f %.3f\n", X1, Y1, r);


  }

  return 0;
}