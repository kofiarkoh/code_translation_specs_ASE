#include <stdio.h>

#define MAXN 2*1000000
#define QQQ 1000000007

typedef unsigned long long ll;

ll S[(MAXN+1)*2];
int V[MAXN+1], H[MAXN+1];
int N;

int init(int n )
{
  int i,j;
  i = 1;
  while ( i < n+1 ) i *= 2;

  for ( j = 0; j < 2*i-1; j++ ) S[j] = 0;

  return i;
}

void update ( int no, ll data, int N )
{
  S[no+N-1] = data;

  int p,t,l,r,maxno;

  p = (no+N-1-1)/2;
  t = 1;

  while ( t > 0 ) {
    t = p;
    l = p*2+1; r = p*2+2;
    maxno = (S[l] > S[r] ) ? l:r;
    if ( S[maxno] == S[p] ) break;
    S[p] = S[maxno];
    p = (p-1)/2;
  }
}

ll find(int x, int y, int no, int l, int r )
{
  if ( r <= x ) return 0;
  if ( l >= y ) return 0;
  if ( l >= x && r <= y ) return S[no];

  int mid = (l+r)/2;
  ll tmp1,tmp2;

  tmp1 = find(x,y,no*2+1,l,mid);
  tmp2 = find(x,y,no*2+2,mid,r);

  return ( tmp1 > tmp2 ) ? tmp1:tmp2;
}

ll flower(int n, int N )
{
  int i,j;
  ll tmp,tmp2,max,pos;

  for ( i = 0; i < n; i++ ) {
    pos = find(0,H[i],0,0,N+1);
    tmp = pos+V[i];
    tmp2 = S[H[i]+N-1];
    if ( tmp > tmp2 ) update(H[i],tmp,N);
  }

  max = S[0+N-1];
  for ( i = 1; i <= n; i++ ) {
    if ( max < S[i+N-1] ) max = S[i+N-1];
  }

  return max;
}

int main(void)
{
  int n,i,tmp;

  scanf("%d",&n);
  for ( i = 0; i < n; i++ ) scanf("%d",&H[i]);
  for ( i = 0; i < n; i++ ) scanf("%d",&V[i]);

  N = init(n);

  printf("%ld\n",flower(n,N));

  return 0;
}
