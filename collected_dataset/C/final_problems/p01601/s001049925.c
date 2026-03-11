#include <stdio.h>
#include <stdlib.h>

static int
palindromicp (
  int x
  )
{
  int s[ 16 ], l;
  int i;

  for ( l = 0; x; ++l, x /= 10 )
    s[ l ] = x % 10;
  for ( i = 0; i <= l / 2; ++i )
    if ( s[ i ] != s[ l - i - 1 ] )
      return ( 0 );

  return ( 1 );
}

int
main (
  int   argc,
  char *argv[ ]
  )
{
  int n;
  int i;

  scanf ( "%d", &n );

  for ( i = 0; ; ++i )
  {
    if ( i <= n
      && palindromicp ( n - i ) )
    {
      printf ( "%d\n", n - i );
      break ;
    }
    if ( palindromicp ( n + i ) )
    {
      printf ( "%d\n", n + i );
      break ;
    }
  }

  return ( EXIT_SUCCESS );
}