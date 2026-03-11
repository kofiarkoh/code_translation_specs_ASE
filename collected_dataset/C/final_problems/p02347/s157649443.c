#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int np;

typedef struct node {
  int location;
  int l;
  int r;
} Node;

Node* T;

typedef int Point[3];

Point* P;
int plen;
int *ans;
int alen;

int compint(const void *a, const void *b)
{
  return *(int *)a - *(int *)b;
}

int lessX(const void *a, const void *b)
{
  return (*(Point*)a)[0] - (*(Point*)b)[0];
}

int lessY(const void *a, const void *b)
{
  return (*(Point*)a)[1] - (*(Point*)b)[1];
}

int (*compar[])(const void*, const void*) = { lessX, lessY };

int make2DTree(int l, int r, int depth)
{
  if (l>=r) return -1;
  int mid = (l+r)/2;
  int t = np++;
  if (r-l>1)
    qsort(&P[l], r-l, sizeof(Point), compar[depth % 2]);
  T[t].location = mid;
  T[t].l = make2DTree(l, mid, depth + 1);
  T[t].r = make2DTree(mid+1, r, depth + 1);

  return t;
}


void find(int v, int sx, int tx, int sy, int ty, int depth)
{
  int x = P[T[v].location][0];
  int y = P[T[v].location][1];
  if (sx<=x && x<=tx && sy<=y && y<=ty) {
    ans[alen++] = P[T[v].location][2];
  }
  if (depth%2 == 0) {
    if (T[v].l >= 0 && sx <=  x)   find(T[v].l, sx, tx, sy, ty, depth + 1);
    if (T[v].r >= 0 && x  <= tx)   find(T[v].r, sx, tx, sy, ty, depth + 1);
  } else {
    if (T[v].l >= 0 && sy <=  y)   find(T[v].l, sx, tx, sy, ty, depth + 1);
    if (T[v].r >= 0 && y  <= ty)   find(T[v].r, sx, tx, sy, ty, depth + 1);
  }
}

int main()
{
  int i, j, k, n, q, sx, tx, sy, ty, x, y, z;
  char buf[64], *p;
  buf[63] = 0;
  fgets(buf, 64, stdin);
  for (n=0,p=buf; *p!='\n'; n=n*10+*(p++)-'0');

  plen = n;
  P = (Point*)malloc(sizeof(Point)*n);
  T = (Node*)malloc(sizeof(Node)*n);
  ans = (int*)malloc(sizeof(int)*n);
  alen = 0;
  int sign;
  for (i=0; i<n; i++) {
    fgets(buf, 64, stdin);
    P[i][2]=i;
    sign = 0;
    p = buf;
    if (*p=='-') { sign=1; p++; }
    for (P[i][0]=0; *p!=' '; P[i][0]=P[i][0]*10+*(p++)-'0');
    if (sign) P[i][0]=-P[i][0];
    sign=0;
    if (*(++p)=='-') { sign=1;   p++; }
    for (P[i][1]=0; *p!='\n'; P[i][1]=P[i][1]*10+*(p++)-'0');
    if (sign) P[i][1]=-P[i][1];
    T[i].l = T[i].r = -1;
  }

  np = 0;
  int root = make2DTree(0, n, 0);
  
  fgets(buf, 64, stdin);
  for (q=0,p=buf; *p!='\n'; q=q*10+*(p++)-'0');

  while (fgets(buf, 64, stdin)) {
    sign = 0;
    p = buf;
    if (*p=='-') { sign=1; p++; }
    for (sx=0; *p!=' '; sx=sx*10+*(p++)-'0');
    if (sign) sx = -sx;
    
    sign = 0; p++;
    if (*p=='-') { sign=1; p++; }
    for (tx=0; *p!=' '; tx=tx*10+*(p++)-'0');
    if (sign) tx=-tx;

    sign = 0;
    if (*(++p)=='-') { sign=1; p++; }
    for (sy=0; *p!=' '; sy=sy*10+*(p++)-'0');
    if (sign) sy=-sy;

    sign = 0;
    if (*(++p)=='-') { sign=1; p++; }
    for (ty=0; *p!='\n'; ty=ty*10+*(p++)-'0');
    if (sign) ty=-ty;

    alen = 0;
    find(root, sx, tx ,sy, ty, 0);
    if (alen>1)
      qsort(ans, alen, sizeof(int), compint);
    for (i=0; i<alen; i++) {
      int d = ans[i];
      for (j=1; j*10<=d; j*=10);
      for (;j>0; j/=10) {
	putc_unlocked('0'+d/j, stdout);
	d %= j;
      }
      putc_unlocked('\n', stdout);
    }
    putc_unlocked('\n', stdout);
    alen = 0;
  }

  return 0;
}

