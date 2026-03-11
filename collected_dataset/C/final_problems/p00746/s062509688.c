#include <stdio.h>

#define N 200
#define MIN(A,B) \
  ( (A) < (B) ? (A) : (B) )
#define MAX(A,B) \
  ( (A) > (B) ? (A) : (B) )

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  int i;

  for ( ; ; )
  {
    int x[ N ] = { 0 }, y[ N ] = { 0 };
    int l = 0, r = 0
      , t = 0, b = 0;
    int n;

    scanf ( "%d", &n );
    if ( !( n ) ) break ;

    for ( i = 1; i < n; ++i )
    {
      int no, d;

      scanf ( "%d%d", &no, &d );
      x[ i ] = x[ no ]; y[ i ] = y[ no ];
      switch ( d )
      {
        case 0: --x[ i ]; break ;
        case 1: ++y[ i ]; break ;
        case 2: ++x[ i ]; break ;
        case 3: --y[ i ]; break ;
      }

      l = MIN( l, x[ i ] ); r = MAX( r, x[ i ] );
      t = MIN( t, y[ i ] ); b = MAX( b, y[ i ] );
    }

    printf ( "%d %d\n", r - l + 1, b - t + 1 );
  }

  return ( 0 );
}