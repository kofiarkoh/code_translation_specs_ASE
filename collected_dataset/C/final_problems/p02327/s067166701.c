#include<stdio.h>
#define MAX 1400
struct rect {
  int height;
  int pos;
};
int buffer[MAX][MAX], table[MAX][MAX];
int large_rectangle(int w, int *t);
int main(void)
{
  int i, j, h, w, max = 0;
  scanf("%d%d", &h, &w);
  for (i = 0; i < h; i++) {
    for (j = 0; j < w; j++) {
      scanf("%d", &buffer[i][j]);
    }
  }
  for (i = 0; i < h; i++) {
    for (j = 0; j < w; j++) {
      if (buffer[i][j]) table[i][j] = 0;
      else table[i][j] = (i > 0) ? table[i-1][j] + 1 : 1;
    }
  }
  for (i = 0; i < h; i++) {
    int temp = large_rectangle(w, table[i]);
    if (max < temp) max = temp;
  }
  printf("%d\n", max);
}
int large_rectangle(int w, int *t)
{
  int i, sp = 0, max = 0;
  struct rect stack[MAX];
  t[w] = 0;
  for (i = 0; i <= w; i++) {
    struct rect r;
    r.height = t[i];
    r.pos = i;
    if (!sp || stack[sp-1].height < r.height) stack[sp++] = r;
    else if (stack[sp-1].height > r.height){
      int target = i;
        while (sp && stack[sp-1].height >= r.height) {
        struct rect pre = stack[--sp];
          int area = pre.height * (i - pre.pos);
          if (max < area) max = area;
          target = pre.pos;
        }
        r.pos = target;
        stack[sp++] = r;
    }
  }
  return max;
}
