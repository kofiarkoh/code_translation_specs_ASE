#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t RUQ_val;

typedef struct range_update_query {
  RUQ_val *val;
  int32_t size;
  int32_t bit;
  RUQ_val empty;
} RUQ;

RUQ* new_RUQ (const int32_t n) {
  int32_t k = 0;
  while ((1 << k) < n) ++k;
  RUQ *s = (RUQ *) calloc (1, sizeof (RUQ));
  s->val = (RUQ_val *) calloc (2 << k, sizeof (RUQ_val));
  s->size = 1 << k;
  s->bit = k;
  s->empty = -1;
  return s;
}

static inline void propagate (RUQ *s, int32_t k) {
  if (s->val[k] == s->empty) return;
  s->val[2 * k] = s->val[2 * k + 1] = s->val[k];
  s->val[k] = s->empty;
}

void update (RUQ *s, int32_t l, int32_t r, RUQ_val v) {
  for (int32_t i = s->bit; i > 0; --i) {
    propagate (s, (l + s->size) >> i);
    propagate (s, (r - 1 + s->size) >> i);
  }
  for (l += s->size, r += s->size; l < r; l >>= 1, r >>= 1) {
    if (l & 1) s->val[l++] = v;
    if (r & 1) s->val[--r] = v;
  }
}

RUQ_val find (RUQ *s, int32_t x) {
  x += s->size;
  for (int32_t i = s->bit; i >= 0; --i) {
    if (s->val[x >> i] != s->empty) {
      return s->val[x >> i];
    }
  }
  return s->empty;
}

typedef int32_t i32;

void run(void){
  i32 n, q;
  scanf ("%" SCNi32 "%" SCNi32, &n, &q);
  RUQ *s = new_RUQ (n);
  update (s, 0, n, 2147483647);
  while (q--) {
    i32 type;
    scanf ("%" SCNi32, &type);
    if (type == 0) {
      i32 l, r, x;
      scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &l, &r, &x);
      update (s, l, r + 1, x);
    } else {
      i32 k;
      scanf ("%" SCNi32, &k);
      printf ("%" PRIi32 "\n", find (s, k));
    }
  }
}

int main(void){
  run();
  return 0;
}

