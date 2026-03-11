#include <stdio.h>
#include <stdlib.h>

#define min(a, b) ((a) > (b) ? (b) : (a))
#define BUF_SIZE 50
int main(void) {
  char line[BUF_SIZE];
  FILE *fp = stdin;
  if(!fgets(line, BUF_SIZE, fp)) return 0;
  int a, b, ab;
  int x, y;
  sscanf(line, "%d %d %d %d %d", &a, &b, &ab, &x, &y);
  if(a > b) {
    int tmp = a;
    a = b;
    b = a;
    tmp = x;
    x = y; 
    y = x;
  }
  // assume that a <= b
  int cost;
  if(ab*2 >= a + b) {
    cost = a*x + b*y;
  } else {
    int ma = min(ab*2, a);
    int mb = min(ab*2, b);
    if(x > y) {
      cost = y*ab*2 + (x-y)*ma;
    } else {
      cost = x*ab*2 + (y - x)*mb;
    }
  }
  printf("%d\n", cost);

  return 0;
}