#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

typedef struct node {
  i32 index;
  i64 val;
} node;

typedef node deque_val;

typedef struct Deque {
  deque_val *array;
  size_t front;
  size_t last;
  size_t mask;
} Deque;

Deque* new_deque (void) {
  const size_t len = 2;
  Deque *d = (Deque *) calloc (len, sizeof (Deque));
  d->array = (deque_val *) calloc (len, sizeof (deque_val));
  d->front = d->last = 0;
  d->mask = len - 1;
  return d;
}

void init_deque (Deque *d) {
  d->front = d->last = 0;
}

void free_deque (Deque *d) {
  free (d->array);
  free (d);
}

int is_empty (Deque *d) {
  return d->front == d->last;
}

size_t get_size (Deque *d) {
  return (d->last + (~d->front + 1)) & d->mask;
}

void deque_realloc (Deque *d) {
  deque_val *array = (deque_val *) calloc (2 * (d->mask + 1), sizeof (deque_val));
  size_t k = 0;
  for (size_t i = d->front; i != d->last; i = (i + 1) & d->mask) {
    array[k++] = d->array[i];
  }
  free (d->array);
  d->array = array;
  d->front = 0;
  d->last = k;
  d->mask = 2 * d->mask + 1;
}

deque_val pop_front (Deque *d) {
  deque_val res = d->array[d->front];
  d->front = (d->front + 1) & d->mask;
  return res;
}

deque_val pop_back (Deque *d) {
  d->last = (d->last + d->mask) & d->mask;
  return d->array[d->last];
}

deque_val get_front (const Deque *d) {
  return d->array[d->front];
}

deque_val get_back (const Deque *d) {
  return d->array[(d->last + d->mask) & d->mask];
}

void push_front (Deque *d, const deque_val v) {
  if(((d->last + 1) & d->mask) == d->front) {
    deque_realloc (d);
  }
  d->front = (d->front + d->mask) & d->mask;
  d->array[d->front] = v;
}

void push_back (Deque *d, const deque_val v) {
  if(((d->last + 1) & d->mask) == d->front) {
    deque_realloc (d);
  }
  d->array[d->last] = v;
  d->last = (d->last + 1) & d->mask;
}

void run (void) {
  i32 n, m, k;
  scanf ("%" SCNi32 "%" SCNi32 "%" SCNi32, &n, &m, &k);
  i32 *a = (i32 *) calloc (n + 1, sizeof (i32));
  for (i32 i = 1; i <= n; ++i) {
    scanf ("%" SCNi32, a + i);
  }
  i64 *dp = (i64 *) calloc (n + 1, sizeof (i64));
  Deque *d = new_deque ();
  for (i32 i = 1; i <= k; ++i) {
    init_deque (d);
    push_back (d, (node){i - 1, dp[i - 1]});
    for (i32 j = i; j <= n; ++j) {
      node t = get_front (d);
      if (t.index < j - m) {
	pop_front (d);
	t = get_front (d);
      }
      i64 local = (i64) i * a[j] + t.val;
      while (!is_empty (d)) {
	t = get_back (d);
	if (t.val > dp[j]) break;
	pop_back (d);
      }
      push_back (d, (node) {j, dp[j]});
      dp[j] = local;
    }
  }
  i64 ans = 0;
  for (i32 i = n; i >= k; --i) {
    if (ans < dp[i]) {
      ans = dp[i];
    }
  }
  printf ("%" PRIi64 "\n", ans);
}

int main (void) {
  run ();
  return 0;
}
