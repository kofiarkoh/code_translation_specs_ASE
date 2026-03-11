#include<stdio.h>
#include<stdlib.h>
#define EPS 1e-8
#define Size 101
struct point {
  double x;
  double y;
};
struct point poly[Size];
int constrain(int n, struct point p);
int main(void)
{
  int i, n, q;
  struct point qoly;
  scanf("%d", &n);
  for (i = 0; i < n; i++) scanf("%lf%lf", &poly[i].x, &poly[i].y);
  poly[n] = poly[0];
  scanf("%d", &q);
  for (i = 0; i < q; i++) {
    scanf("%lf%lf", &qoly.x, &qoly.y);
    printf("%d\n", constrain(n, qoly));
  }
}
int constrain(int n, struct point p)
{
  int i, x = 0;
  double cross, dot;
  struct point a, b;
  for (i = 0; i < n; i++) {
    a.x = poly[i].x - p.x, a.y = poly[i].y - p.y;
    b.x = poly[i+1].x - p.x, b.y = poly[i+1].y - p.y;
    cross = a.x * b.y - b.x * a.y;
    dot = a.x * b.x + a.y * b.y;
    if (abs(cross) < EPS && dot < EPS) return 1;
    if (a.y > b.y) {
      struct point temp = a;
      a = b;
      b = temp;
      cross *= -1;
    }
    if (a.y < EPS && b.y > EPS && cross > EPS) x = !x;
  }
  return (x % 2 ? 2 : 0);
}
