#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;
typedef uint64_t u64;

typedef struct union_find_add {
  i32 *parent;
  i64 *add;
} union_find;

union_find* new_union_find (i32 n) {
  union_find *u = (union_find *) calloc (1, sizeof (union_find));
  u->parent = (i32 *) calloc (n, sizeof (i32));
  u->add = (i64 *) calloc (n, sizeof (i64));
  for (i32 i = 0; i < n; ++i) {
    u->parent[i] = -1;
  }
  return u;
}

i32 root (union_find *u, i32 x) {
  i32 stack[32];
  i32 top = 0;
  stack[top++] = x;
  while (u->parent[x] >= 0) {
    x = u->parent[x];
    stack[top++] = x;
  }
  i32 r = stack[top - 1];
  i64 buf = 0;
  for (i32 i = top - 2; i >= 0; --i) {
    i32 y = stack[i];
    i64 next = buf + u->add[y];
    u->add[y] += buf;
    buf = next;
    u->parent[y] = r;
  }
  return r;
}

void unite (union_find *u, i32 x, i32 y) {
  x = root (u, x);
  y = root (u, y);
  if (x == y) return;
  if (u->parent[x] > u->parent[y]) {
    i32 s = x;
    x = y;
    y = s;
  }
  u->add[y] -= u->add[x];
  u->parent[x] += u->parent[y];
  u->parent[y] = x;
}

void add (union_find *u, i32 x, i64 v) {
  u->add[root (u, x)] += v;
}

i64 find (union_find *u, i32 x) {
  i32 r = root (u, x);
  return r == x ? u->add[x] : u->add[x] + u->add[r];
}

void radix_sort (u64 *a, i32 n) {
  const i32 w = 5;
  const i32 mask = (1 << w) - 1;
  u64 *val = (u64 *) calloc (n, sizeof (u64));
  i32 *next = (i32 *) calloc (n + (1 << w), sizeof (i32));
  i32 *start = next + n;
  for (i32 b = 34; b < 64; b += w) {
    for (i32 j = 0; j < (1 << w); ++j) {
      start[j] = -1;
    }
    for (i32 i = 0; i < n; ++i) {
      i32 k = (a[i] >> b) & mask;
      val[i] = a[i];
      next[i] = start[k];
      start[k] = i;
    }
    for (i32 i = n - 1, k = mask; k >= 0; --k) {
      for (i32 p = start[k]; p != -1; p = next[p]) {
        a[i--] = val[p];
      }
    }
  }
  free (val);
  free (next);
}

static void print_int(i64 n){if(n<0){putchar('-');n=-n;}if(n==0){putchar('0');return;}int s[20],len=0;while(n>0){s[len++]=n%10+'0';n/=10;}while(len>0){putchar(s[--len]);}}
static i64 read_int(void){int prev='\0';int c=getchar();while(!('0'<=c && c<='9')){prev=c;c=getchar();}i64 res=0;while('0'<=c && c<='9'){res=10*res+c-'0';c=getchar();}return prev=='-'?-res:res;}

void run (void) {
  i32 n;
  scanf ("%" SCNi32, &n);
  u64 *e = (u64 *) calloc (n - 1, sizeof (u64));
  const i32 f = 17;
  const i32 mask = (1 << f) - 1;
  for (i32 i = 0; i < n - 1; ++i) {
    i32 a = read_int() - 1;
    i32 b = read_int() - 1;
    i32 c = read_int();
    e[i] = ((u64) c << 2 * f) + ((u64) a << f) + b;
  }
  radix_sort (e, n - 1);
  union_find *u = new_union_find (n);
  for (i32 i = n - 2; i >= 0; --i) {
    i32 a = root (u, e[i] & mask);
    i32 b = root (u, e[i] >> f & mask);
    i64 c = e[i] >> 2 * f;
    add (u, b, -u->parent[a] * c);
    add (u, a, -u->parent[b] * c);
    unite (u, a, b);
  }
  for (i32 i = 0; i < n; ++i) {
    print_int (find (u, i));
    putchar ('\n');
  }
}

int main (void) {
  run();
  return 0;
}
