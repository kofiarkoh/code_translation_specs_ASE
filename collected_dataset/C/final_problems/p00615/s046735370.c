#include <stdio.h>

#define N 10000
#define MAX(A,B) \
  ( (B) > (A) ? (B) : (A) )

/** Application main entry point. */
int
main (
  int     argc,
  char  * argv[ ]
  )
{
  int i, j;

  for ( ; ; )
  {
    int d[ N * 2 ];
    int n, m;
    int res;

    scanf ( "%d%d", &n, &m );
    if ( !( n | m ) ) break ;
    m += n;

    for ( i = 0; i < n; ++i )
    {
      scanf ( "%d", &d[ i ] );
    }
    for ( i = n; i < m; ++i )
    {
      scanf ( "%d", &d[ i ] );
    }
    for ( i = 0; i < n && n < m; ++i )
    {
      if ( d[ n ] < d[ i ] )
      {
        const int t = d[ i ];

        d[ i ] = d[ n ];
        for ( j = n; j + 1 < m && d[ j + 1 ] < t; ++j )
        {
          d[ j ] = d[ j + 1 ];
        }
        d[ j ] = t;
      }
      else
      {
        ++n;
      }
    }

    res = d[ 0 ];
    for ( i = 1; i < m; ++i )
    {
      res = MAX( res, d[ i ] - d[ i - 1 ] );
    }
    printf ( "%d\n", res );
  }

  return ( 0 );
}