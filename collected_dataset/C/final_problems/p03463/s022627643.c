#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUF_SIZE 15
int main(void) {
  char line[BUF_SIZE];
  FILE *fp = stdin;
  if(!fgets(line, BUF_SIZE, fp)) return 0;
  int num, a, b;
  sscanf(line, "%d %d %d", &num, &a, &b);
  int res = !((b - a)&1);
  // if(num >= 3 && b==num && a == num-2) res = 1;
  // if(num==2) res = 0;
  printf("%s\n", res ? "Alice" : "Borys");
  return 0;
}