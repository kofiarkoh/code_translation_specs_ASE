#include <stdio.h>

#define MAXN 100002
#define QQQ 1000000007

typedef struct AAA {
  int cnt;
  int c[300];
} NODE;

typedef struct BBB {
   long b,w;
} DP;

NODE A[MAXN+1];
DP S[MAXN+1];

DP indset ( int no, int p )
{
  if ( S[no].w != -1 ) return S[no];

  long i,b,w;
  DP tmp,ans;

  ans.w = ans.b = 1;
  for ( i = 0; i < A[no].cnt; i++ ) {
    if ( A[no].c[i] == p ) continue;
    tmp = indset(A[no].c[i], no);
    ans.b *= tmp.w;
    ans.b %= QQQ;
    ans.w *= (tmp.b+tmp.w);
    ans.w %= QQQ;
  }

  return S[no] = ans;
}

int main(void)
{
  int n,i,x,y,j,maxno = 1;
  DP ans;

  scanf("%d", &n);

  for ( i = 0; i <= n; i++ ) {
    A[i].cnt = 0;
    S[i].w = -1;
  }
  for ( i = 0; i < n-1; i++  ){
    scanf("%d%d", &x,&y);
    A[x].c[A[x].cnt++] = y;
    A[y].c[A[y].cnt++] = x;
  }

  ans = indset(1,0);

  printf("%d\n",(ans.b+ans.w)%QQQ);
  return 0;
}
