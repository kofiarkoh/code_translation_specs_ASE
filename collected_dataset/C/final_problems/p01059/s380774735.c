#include<stdio.h>

int main()
{

  int n, m, a[ 100000 ];
  int i;
  int MAX = 0, tmp;

  scanf( "%d%d", &n, &m );

  for( i = 0; i < m; i++ ){
    scanf( "%d", &a[ i ] );
  }

  if( a[ 0 ] != 1 ) MAX = a[ 0 ] - 1;

  for( i = 0; i < m; i++ ){
    tmp = ( a[ i ] - a[ i - 1 ] ) / 2;
    if( MAX < tmp ) MAX = tmp;
  }

  tmp = n - a[ m - 1 ];
  if( MAX < tmp ) MAX = tmp;

  
  printf( "%d\n", MAX );

  return 0;

}