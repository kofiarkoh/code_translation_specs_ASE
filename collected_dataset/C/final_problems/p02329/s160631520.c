#include<stdio.h>
#define HASHSIZE 3456781
#define SIZE 1000
struct hash {
  long long v;
  int f;
};
struct hash h[HASHSIZE], *hend = h+HASHSIZE;
int N;
long long V, V2, V3;
long long a[SIZE], b[SIZE], c[SIZE], d[SIZE];
int Input(long long *x);
void Insert(long long n);
int Lookup(long long n);
int main(void)
{
  int i, j, ca, cb, cc, cd;
  long long temp, ans = 0;
  scanf("%d%lld", &N, &V);
  V2 = V - 2, V3 = V - 3;
  ca = Input(a);
  cb = Input(b);
  cc = Input(c);
  cd = Input(d);
  for (i = 0; i < ca; i++) {
    for (j = 0; j < cb; j++) {
      temp = a[i] + b[j];
      if (temp <= V2) Insert(temp);
    }
  }
  for (i = 0; i < cc; i++) {
    for (j = 0; j < cd; j++) {
      temp = c[i] + d[j];
      if (temp <= V2) ans += Lookup(V - temp);
    }
  }
  printf("%lld\n", ans);
  return 0;
}
int Input(long long *x)
{
  int i, count = 0;
  long long temp;

  for (i = 0; i < N; i++) {
    scanf("%lld", &temp);
    if (temp <= V3) x[count++] = temp;
  }
  return count;
}
void Insert(long long n)
{
  struct hash *p = h + (int)(n % HASHSIZE);
  while (p->v) {
    if (p->v == n) {
      p->f++;
      return;
    }
    if (++p == hend) p = h;
  }
  p->v = n, p->f = 1;
}

int Lookup(long long n)
{
  struct hash *p = h + (int)(n % HASHSIZE);
  while (p->v) {
    if (p->v == n) return p->f;
    if (++p == hend) p = h;
  }
  return 0;
}
