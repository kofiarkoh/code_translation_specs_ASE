#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

typedef struct result {
  i64 n;
  i64 ans;
} result;

typedef result WBT_val;

static inline int compare_WBT_val (const WBT_val a, const WBT_val b) {
  i64 d = a.n - b.n;
  return d == 0 ? 0 : d < 0 ? -1 : 1;;
}

typedef struct weight_balanced_tree_node {
  struct weight_balanced_tree_node *left;
  struct weight_balanced_tree_node *right;
  i32 lsize;
  i32 rsize;
  WBT_val val;
} WBT_node;

WBT_node* new_WBT_node (const WBT_val val) {
  WBT_node *t = (WBT_node *) calloc (1, sizeof (WBT_node));
  t->left = NULL;
  t->right = NULL;
  t->lsize = 0;
  t->rsize = 0;
  t->val = val;
  return t;
}

i32 get_size (const WBT_node * const t) {
  return t == NULL ? 0 : t->lsize + t->rsize + 1;
}

int get_bias (const WBT_node * const t, const i32 b) {
  if (t == NULL) {
    return 0;
  }
  const i32 l = t->lsize + 1;
  const i32 r = t->rsize + 1;
  if (l * b >= r && l <= b * r) {
    return 0;
  }
  return l * b < r ? -1 : 1;
}

void WBT_node_update (WBT_node * const t) {
  if (t == NULL) return;
  t->lsize = get_size (t->left);
  t->rsize = get_size (t->right);
}

WBT_node* left_rotate (WBT_node * const u) {
  WBT_node * const v = u->right;
  u->right = v->left;
  v->left = u;
  WBT_node_update (u);
  WBT_node_update (v);
  return v;
}

WBT_node* right_rotate (WBT_node * const v) {
  WBT_node * const u = v->left;
  v->left = u->right;
  u->right = v;
  WBT_node_update (v);
  WBT_node_update (u);
  return u;
}

WBT_node* balance (WBT_node * const t) {
  i32 b = get_bias (t, 3);
  if (b == 0) {
    return t;
  }
  if (b < 0) {
    if (get_bias (t->right, 2) > 0) {
      t->right = right_rotate (t->right);
    }
    return left_rotate (t);
  } else {
    if (get_bias (t->left, 2) < 0) {
      t->left = left_rotate (t->left);
    }
    return right_rotate (t);
  }
}

WBT_node* insert (WBT_node * const t, const WBT_val val) {
  if (t == NULL) {
    return new_WBT_node (val);
  }
  int c = compare_WBT_val (val, t->val);
  if (c == 0) {
    t->val = val;
    return t;
  }
  if (c < 0) {
    t->left = insert (t->left, val);
  } else {
    t->right = insert (t->right, val);
  }
  WBT_node_update (t);
  return balance (t);
}

WBT_val search (WBT_node * const t, const WBT_val val) {
  if (t == NULL) return val;
  int c = compare_WBT_val (val, t->val);
  if (c == 0) return t->val;
  return c < 0 ? search (t->left, val) : search (t->right, val);
}

#define MIN(a,b) ((a) < (b) ? (a) : (b))

i64 calc (i64 n, i64 a, i64 begin) {
  if (n <= a) return n;
  static WBT_node *memo = NULL;
  result v = search (memo, (result) {n, n + 1});
  if (v.ans <= n) return v.ans;
  i64 min = n;
  for (i64 i = begin; i * i <= n; ++i) {
    i64 d = i;
    i64 x = (n + d - 1) / d;
    i64 t = d + a + calc (x, a, i);
    min = MIN(min, t);
  }
  insert (memo, (result) {n, min});
  return min;
}

void run (void) {
  i64 n, a;
  scanf ("%" SCNi64 "%" SCNi64, &n, &a);
  i64 ans = calc (n, a, 2);
  printf ("%" PRIi64 "\n", ans);
}

int main (void) {
  run();
  return 0;
}
