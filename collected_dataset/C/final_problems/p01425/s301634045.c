#include <stdio.h>
#include <math.h>
#define NMAX 300
#define EPS 1e-10
const double g = 9.8;

int n, v, x, y;
int left[NMAX], bot[NMAX], right[NMAX], top[NMAX];
int cmpi(double lb, double ub, double c);
int check(double qx, double qy);

int main(void)
{
  int i, hit;
  scanf("%d %d %d %d", &n, &v, &x, &y);
  for (i = 0; i < n; i++) {
    scanf("%d %d %d %d", &left[i], &bot[i], &right[i], &top[i]);
    if (right[i]> x) right[i] = x;
  }
  hit = check(x, y);
  for (i = 0; i < n; i++) {
    hit |= check(left[i], top[i]);
    hit |= check(right[i], top[i]);
  }
  printf("%s\n", hit ? "Yes" : "No");
}

int cmpi(double lb, double ub, double c)
{
  return c < lb + EPS ? -1 : c > ub - EPS ? 1 : 0;
}

int check(double qx, double qy)
{
  int i, d, hit = 1;
  double a = g * g / 4, b = g * qy - v * v, c = qx * qx + qy * qy;
  double D = b * b - 4 * a * c;
  if (D < 0 && D > -EPS) D = 0;
  if  (D < 0) return 0;
  for (d = -1; d <= 1; d += 2) {
    double t, vx, vy, yt, t2 = (-b + d * sqrt(D)) / (2 * a);
    if (t2 <= 0) continue;
    t = sqrt(t2);
    vx = qx / t;
    vy = qy / t + g * t / 2;
    yt = vy * x / vx - g * pow(x / vx, 2) / 2;
    if (yt < y - EPS) continue;
    for (i = 0; i < n; i++) {
      int yl, yr, xh, yh;
      if (left[i] >= x) continue;
      if (right[i] == x && y <= top[i] && bot[i] <= yt) {
        hit = 0;
        break;
      }
      yl = cmpi(bot[i], top[i], vy * left[i] / vx - g * pow(left[i] / vx, 2) / 2);
      yr = cmpi(bot[i], top[i], vy * right[i] / vx - g * pow(right[i] / vx, 2) / 2);
      xh = cmpi(left[i], right[i], vx * (vy / g));
      yh = cmpi(bot[i], top[i], vy * vy / g - g * pow(vy / g, 2) / 2);
      if ((xh ==0 && yh >= 0 && yl < 0) || (yl * yr <= 0)) {
        hit = 0;
        break;
      }
    }
    if (hit) return 1;
  }
  return 0;
}
