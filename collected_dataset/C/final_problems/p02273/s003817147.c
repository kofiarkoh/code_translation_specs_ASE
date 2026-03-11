#include <stdio.h>

typedef struct{
  double x, y;
} Point;

void kock(int, Point, Point);

int main(){
  int n;
  Point p1, p2;

  p1.x = 0;
  p1.y = 0;
  p2.x = 100;
  p2.y = 0;

  scanf("%d", &n);

  printf("%.8f %.8f\n", p1.x, p1.y);

  kock(n, p1, p2);

  printf("%.8f %.8f\n", p2.x, p2.y);
  
  return 0;
}

void kock(int n, Point p1, Point p2){
  Point s, t, u;
  double cos60 = 0.5;
  double sin60 = 0.8660254037844386;
  
  if(n == 0) return;

  s.x = (2 * p1.x + 1 * p2.x) / 3;
  s.y = (2 * p1.y + 1 * p2.y) / 3;
  
  t.x = (1 * p1.x + 2 * p2.x) / 3;
  t.y = (1 * p1.y + 2 * p2.y) / 3;

  u.x = (t.x - s.x) * cos60 - (t.y - s.y) * sin60 + s.x;
  u.y = (t.x - s.x) * sin60 + (t.y - s.y) * cos60 + s.y;

  kock(n-1, p1, s);
  printf("%.8f %.8f\n", s.x, s.y);
  kock(n-1, s, u);
  printf("%.8f %.8f\n", u.x, u.y);
  kock(n-1, u, t);
  printf("%.8f %.8f\n", t.x, t.y);
  kock(n-1, t, p2);
}