#include <stdio.h>
#include <stdint.h>

#define SQ(X) \
  ( (X) * (X) )
#define MAX(A,B) \
  ( (A) > (B) ? (A) : (B) )
#define REDUCE(N,D) \
  { const int G__19 = gcd ( N, D ); \
    (N) /= G__19; (D) /= G__19; \
  }

static int
sqrt2 (
  uint64_t a
  )
{
  uint64_t res = MAX( a, 3 );

  while ( a < SQ( res ) || a >= SQ( res + 1 ) )
  {
    res = ( SQ( res ) + a ) / 2 / res;
  }

  return ( res );
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
      int t = sqrt2 ( p * SQ( i ) );

      if ( t > n ) continue ;

      if ( t * ld > ln * i )
        { ln = t; ld = i; }
      if ( t < n && ( t + 1 ) * ud < un * i )
        { un = t + 1; ud = i; }
    }

    REDUCE( ln, ld ); REDUCE( un, ud );
    printf ( "%d/%d %d/%d\n", un, ud, ln, ld );
  }

  return ( 0 );
}