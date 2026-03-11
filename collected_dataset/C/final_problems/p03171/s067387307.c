#include <stdio.h>

#define MAXN 300

long S[MAXN][MAXN];
int T[MAXN][MAXN];
int A[MAXN];

long deque(int i, int j, int cnt )
{
  if ( T[i][j] ==  1 ) return S[i][j];
  T[i][j] = 1;
  if ( i == j ) {
    if ( cnt == 1 ) return S[i][j] = A[i];
    else return S[i][j] = -A[i];
  }

  long tmp1, tmp2;
  if ( cnt == 1 ) {
    tmp1 = deque(i+1, j, (cnt+1)%2 ) + A[i];
    tmp2 = deque(i, j-1, (cnt+1)%2 ) + A[j];
    return S[i][j] = (tmp1 > tmp2 ) ? tmp1:tmp2;
  } else {
    tmp1 = deque(i+1, j, (cnt+1)%2 ) - A[i];
    tmp2 = deque(i, j-1, (cnt+1)%2 ) - A[j];
    return S[i][j] = (tmp1 < tmp2 ) ? tmp1:tmp2;
  }
}

int main(void)
{
  int i,n,j;

  scanf("%d", &n);
  for ( i = 0; i < n; i++ ) scanf("%d", &A[i]);

  for ( i = 0; i < n; i++ ) for ( j = 0; j < n; j++ ) T[i][j] = 0;

  printf("%ld\n", deque(0,n-1,1));

  return 0;

}
