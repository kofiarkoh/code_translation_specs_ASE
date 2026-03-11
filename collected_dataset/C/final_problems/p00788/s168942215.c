#include <stdio.h>

#define SQ(X) \
  ( (X) * (X) )
#define REDUCE(N,D) \
  { const int G__19 = gcd ( N, D ); \
    (N) /= G__19; (D) /= G__19; \
  }

static int
gcd (
  int a,
  int b
  )
{
  do
  {
    const int t = a;
    a = b % a;
    b = t;
  }
  while ( a );

  return ( b );
}

/** Application main entry point. */
int
main (
  int     argc,
  char  * argv[ ]
  )
{
  int i;

  for ( ; ; )
  {
    int ln = 0,     ld = 1
      , un = 10000, ud = 1;
    int p, n;

    scanf ( "%d%d", &p, &n );
    if ( !( p | n ) ) break ;

    for ( i = 1; i <= n; ++i )
    {
      const int det = p * SQ( i );
      int a, b;

      for ( a = 1, b = n; b - a > 1; )
      {
        const int m = ( a + b ) / 2;
        if ( SQ( m ) < det )  a = m;
        else                  b = m;
      }
      if ( SQ( a ) < det && a * ld > ln * i )
        { ln = a;     ld = i; }
      if ( SQ( b ) > det && b * ud < un * i )
        { un = b; ud = i; }
    }

    REDUCE( ln, ld ); REDUCE( un, ud );
    printf ( "%d/%d %d/%d\n", un, ud, ln, ld );
  }

  return ( 0 );
}