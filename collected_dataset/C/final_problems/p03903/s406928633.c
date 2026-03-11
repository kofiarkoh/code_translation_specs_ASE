#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
#include<inttypes.h>

typedef int32_t i32;
typedef int64_t i64;

typedef struct quick_find {
  i32 *id;
  i32 **elem;
  i32 *size;
  i32 *max_size;
  i32 num;
} quick_find;

quick_find* new_quick_find (const i32 n) {
  quick_find *q = (quick_find *) calloc (1, sizeof (quick_find));
  q->id = (i32 *) calloc (n, sizeof (i32));
  q->elem = (i32 **) calloc (n, sizeof (i32 *));
  q->size = (i32 *) calloc (n, sizeof (i32));
  q->max_size = (i32 *) calloc (n, sizeof (i32));
  q->num = n;
  for (i32 i = 0; i < n; ++i) {
    q->id[i] = i;
    q->elem[i] = (i32 *) calloc (1, sizeof (i32));
    q->elem[i][0] = i;
    q->size[i] = 1;
    q->max_size[i] = 1;
  }
  return q;
}

i32 get_id (quick_find *q, i32 x) {
  return q->id[x];
}

i32 get_size (quick_find *q, i32 x) {
  return q->size[get_id (q, x)];
}

int same (quick_find *q, i32 x, i32 y) {
  return get_id (q, x) == get_id (q, y);
}

void unite (quick_find *q, i32 x, i32 y) {
  x = get_id (q, x);
  y = get_id (q, y);
  if (x == y) {
    return;
  }
  if (q->size[x] < q->size[y]) {
    i32 s = x;
    x = y;
    y = s;
  }
  if (q->max_size[x] < q->size[x] + q->size[y]) {
    q->max_size[x] *= 2;
    q->elem[x] = realloc (q->elem[x], sizeof (i32) * q->max_size[x]);
  }
  for (i32 i = 0; i < q->size[y]; ++i) {
    i32 v = q->elem[y][i];
    q->id[v] = x;
    q->elem[x][q->size[x]++] = v;
  }
  free (q->elem[y]);
  q->elem[y] = NULL;
  q->size[y] = 0;
}

static void print_int(i64 n){if(n<0){putchar('-');n=-n;}if(n==0){putchar('0');return;}int s[20],len=0;while(n>0){s[len++]=n%10+'0';n/=10;}while(len>0){putchar(s[--len]);}}
static i64 read_int(void){int prev='\0';int c=getchar();while(!('0'<=c && c<='9')){prev=c;c=getchar();}i64 res=0;while('0'<=c && c<='9'){res=10*res+c-'0';c=getchar();}return prev=='-'?-res:res;}

int cmp (const void *a, const void *b) {
  i64 d = *(const i64 *)a - *(const i64 *)b;
  return d == 0 ? 0 : d < 0 ? -1 : 1;
}

void run (void) {
  i32 n = read_int();
  i32 m = read_int();
  i64 *e = (i64 *) calloc (m, sizeof (i64));
  for (i32 i = 0; i < m; ++i) {
    i32 a = read_int() - 1;
    i32 b = read_int() - 1;
    i32 c = read_int();
    e[i] = ((i64)c << 30) + a * n + b;
  }
  qsort (e, m, sizeof (i64), cmp);
  i32 *ans = (i32 *) calloc (n * n, sizeof (i32));
  i64 sum = 0;
  quick_find *q = new_quick_find (n);
  for (i32 i = 0; i < m; ++i) {
    i32 a = (e[i] & ((1 << 30) - 1)) / n;
    i32 b = (e[i] & ((1 << 30) - 1)) % n;
    if (same (q, a, b)) continue;
    a = get_id (q, a);
    b = get_id (q, b);
    i32 c = e[i] >> 30;
    sum += c;
    for (i32 j = 0; j < q->size[a]; ++j) {
      i32 s = q->elem[a][j];
      for (i32 k = 0; k < q->size[b]; ++k) {
        i32 t = q->elem[b][k];
        ans[s * n + t] = ans[t * n + s] = c;
      }
    }
    unite (q, a, b);
  }
  i32 iter = read_int();
  while (iter--) {
    i32 s = read_int() - 1;
    i32 t = read_int() - 1;
    print_int (sum - ans[s * n + t]);
    putchar ('\n');
  }
}

int main (void) {
  run();
  return 0;
}
