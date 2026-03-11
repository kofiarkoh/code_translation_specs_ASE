#include <stdio.h>
#include <string.h>
#define MAX 1001

int main() {
  int q, k, c[MAX][MAX], i, j, m, n;
  char X[MAX], Y[MAX];
  
  scanf( "%d" , &q );
  for ( k=0; k<q; k++ ) {
    scanf( "%s" , X );
    scanf( "%s" , Y );
    m = strlen(X);
    n = strlen(Y);

    for ( i=1; i<=m; i++ ) c[i][0] = 0;
    for ( j=1; j<=n; j++ ) c[0][j] = 0;
    for ( i=1; i<=m; i++ )
      for ( j=1; j<=n; j++ ) {
	if ( X[i-1] == Y[j-1] ) c[i][j] = c[i-1][j-1] + 1;
	else if ( c[i-1][j] >= c[i][j-1] ) c[i][j] = c[i-1][j];
	else c[i][j] = c[i][j-1];
      }

    printf( "%d\n" , c[m][n] );
  }
  
  return 0;
}



