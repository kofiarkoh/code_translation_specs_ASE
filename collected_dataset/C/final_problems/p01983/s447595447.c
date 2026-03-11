#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>
#include<string.h>

typedef int32_t i32;

#define ALLOC(size,type) ((type*)calloc((size),sizeof(type)))

i32 eval (char *s, i32 *x, i32 *k) {
  if (s[*k] != '[') {
    i32 res = x[s[*k] - 'a'];
    *k += 1;
    return res;
  }
  i32 op = s[*k + 1];
  *k += 2;//[ op
  i32 l = eval (s, x, k);
  i32 r = eval (s, x, k);
  i32 res = 0;
  if (op == '+') {
    res = l | r;
  } else if (op == '*') {
    res = l & r;
  } else {
    res = l ^ r;
  }
  *k += 1;//]
  return res;
}

void loop (i32 p, i32 *x, char *s, i32 *cnt) {
  if (p == 4) {
    i32 k = 0;
    cnt[eval (s, x, &k)]++;
    return;
  }
  for (x[p] = 0; x[p] < 10; x[p]++) {
    loop (p + 1, x, s, cnt);
  }
}

void run (void) {
  char *s = ALLOC (80 + 1, char);
  char *d = ALLOC (4 + 1, char);
  i32 *cnt = ALLOC (16, i32);
  for (;;) {
    scanf ("%s", s);
    if (s[0] == '.') break;
    scanf ("%s", d);
    i32 x[4];
    for (i32 i = 0; i < 4; ++i) {
      x[i] = d[i] - '0';
    }
    i32 k = 0;
    const i32 v = eval (s, x, &k);
    memset (cnt, 0, sizeof (i32) * 16);
    loop (0, x, s, cnt);
    printf ("%" PRIi32 " %" PRIi32 "\n", v, cnt[v]);
  }
}

int main (void) {
  run();
  return 0;
}

