#include <stdio.h>

#define MAX(A,B) \
  ( (A) > (B) ? (A) : (B) )

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  int i, j;

  for ( ; ; )
  {
    int res = 0;
    int ti, tj;
    int n;

    scanf ( "%d", &n );
    if ( !( n ) ) break ;

    for ( j = 0; ( tj = j * j * j ) <= n; ++j )
    for ( i = 0; ( ti = i * ( i + 1 ) * ( i + 2 ) / 6 + tj ) <= n; ++i )
    {
      res = MAX( res, ti );
    }

    printf ( "%d\n", res );
  }

  return ( 0 );
}