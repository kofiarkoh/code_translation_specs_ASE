#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

typedef struct RSQandRUQ_node {
  i32 val;
  i32 sum;
} node;

typedef struct RSQandRUQ {
  node *a;
  i32 bit;
  i32 size;
  i32 empty;
} RSQandRUQ;

RSQandRUQ* new_RSQandRUQ (const i32 n) {
  i32 k = 0;
  while ((1 << k) < n) ++k;
  RSQandRUQ *s = (RSQandRUQ *) calloc (1, sizeof (RSQandRUQ));
  s->a = (node *) calloc (2 << k, sizeof (node));
  s->bit = k;
  s->size = 1 << k;
  s->empty = 10000;
  return s;
}

void propagate (RSQandRUQ *s, i32 x) {
  x += s->size;
  for (i32 i = s->bit; i > 0; --i) {
    i32 k = x >> i;
    if (s->a[k].val == s->empty) continue;
    s->a[2 * k].val = s->a[2 * k + 1].val = s->a[k].val;
    s->a[k].sum = s->a[k].val << i;
    s->a[k].val = s->empty;
  }
}

static inline i32 eval (RSQandRUQ *s, i32 k, i32 len) {
  return s->a[k].val == s->empty ? s->a[k].sum : s->a[k].val * len;
}

void save (RSQandRUQ *s, i32 x) {
  for (i32 k = (x + s->size) >> 1, len = 1; k > 0; k >>= 1, len <<= 1) {
    s->a[k].sum = eval (s, 2 * k, len) + eval (s, 2 * k + 1, len);
  }
}

void update (RSQandRUQ *s, i32 l, i32 r, i32 v) {
  propagate (s, l);
  propagate (s, r - 1);
  for (i32 x = l + s->size, y = r + s->size; x < y; x >>= 1, y >>= 1) {
    if (x & 1) s->a[x++].val = v;
    if (y & 1) s->a[--y].val = v;
  }
  save (s, l);
  save (s, r - 1);
}

i32 get_sum (RSQandRUQ *s, i32 l, i32 r) {
  propagate (s, l);
  propagate (s, r - 1);
  i32 sum = 0;
  for (i32 x = l + s->size, y = r + s->size, len = 1; x < y; x >>=1, y >>= 1, len <<= 1) {
    if (x & 1) sum += eval (s, x++, len);
    if (y & 1) sum += eval (s, --y, len);
  }
  return sum;
}

void run (void) {
  i32 n, q;
  scanf ("%" SCNi32 "%" SCNi32, &n, &q);
  RSQandRUQ *s = new_RSQandRUQ (n);
  while (q--) {
    i32 t, l, r;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &t,&l, &r);
    r++;
    if (t == 0) {
      i32 v;
      scanf ("%" SCNi32, &v);
      update (s, l, r, v);
    } else {
      printf ("%" PRIi32 "\n", get_sum (s, l, r));
    }
  }
}

int main(void){
  run();
  return 0;
}

