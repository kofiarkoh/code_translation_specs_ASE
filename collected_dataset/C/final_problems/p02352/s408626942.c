#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;

typedef struct RMQandRAQ {
  i32 *add;
  i32 *min;
  i32 bit;
  i32 size;
} RMQandRAQ;

RMQandRAQ* new_RMQandRAQ (const i32 n) {
  i32 k = 0;
  while ((1 << k) < n) ++k;
  RMQandRAQ *s = (RMQandRAQ *) calloc (1, sizeof (RMQandRAQ));
  s->add = (i32 *) calloc (2 << k, sizeof (i32));
  s->min = (i32 *) calloc (2 << k, sizeof (i32));
  s->bit = k;
  s->size = 1 << k;
  return s;
}

static inline i32 min (i32 a, i32 b) {
  return a < b ? a : b;
}

static inline i32 eval (RMQandRAQ *s, i32 k) {
  return s->add[k] + s->min[k];
}

void propagate (RMQandRAQ *s, i32 x) {
  x += s->size;
  for (i32 bit = s->bit; bit > 0; bit--) {
    i32 k = x >> bit;
    s->add[2 * k] += s->add[k];
    s->add[2 * k + 1] += s->add[k];
    s->add[k] = 0;
    s->min[k] = min (eval (s, 2 * k), eval (s, 2 * k + 1));
  }
}

void save (RMQandRAQ *s, i32 x) {
  for (i32 k = (x + s->size) >> 1; k >= 1; k >>= 1) {
    s->min[k] = min (eval (s, 2 * k), eval (s, 2 * k + 1));
  }
}

void add (RMQandRAQ *s, i32 l, i32 r, i32 v) {
  propagate (s, l);
  propagate (s, r - 1);
  for (i32 x = l + s->size, y = r + s->size; x < y; x >>= 1, y >>= 1) {
    if (x & 1) s->add[x++] += v;
    if (y & 1) s->add[--y] += v;
  }
  save (s, l);
  save (s, r - 1);
}

i32 find (RMQandRAQ *s, i32 l, i32 r) {
  propagate (s, l);
  propagate (s, r - 1);
  i32 ans = 1000 * 100000 + 1;
  for (l += s->size, r += s->size; l < r; l >>= 1, r >>= 1) {
    if (l & 1) ans = min (ans, eval (s, l++));
    if (r & 1) ans = min (ans, eval (s, --r));
  }
  return ans;
}

void run (void) {
  i32 n, q;
  scanf ("%" SCNi32 "%" SCNi32, &n, &q);
  RMQandRAQ *s = new_RMQandRAQ (n);
  while (q--) {
    i32 t, l, r;
    scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &t, &l, &r);
    r++;
    if (t == 0) {
      i32 v;
      scanf ("%" SCNi32, &v);
      add (s, l, r, v);
    } else {
      printf ("%" PRIi32 "\n", find (s, l, r));
    }
  }
}

int main (void) {
  run ();
  return 0;
}

