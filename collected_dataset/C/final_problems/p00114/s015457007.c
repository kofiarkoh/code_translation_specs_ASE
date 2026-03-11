#include <stdio.h>
#include <stdint.h>

static int64_t
gcd (
  int64_t a,
  int64_t b
  )
{
  do
  {
    const int64_t t = a;
    a = b % a;
    b = t;
  }
  while ( a );

  return ( b );
}

static int64_t
lcm (
  int64_t a,
  int64_t b
  )
{
  return ( a * b / gcd ( a, b ) );
}

/** Application main entry point. */
int
main (
  int     argc,
  char  * argv[ ]
  )
{
  for ( ; ; )
  {
    int64_t a1, m1, a2, m2, a3, m3;
    int64_t r1, c1, r2, c2, r3, c3;

    scanf ( "%lld%lld%lld%lld%lld%lld", &a1, &m1, &a2, &m2, &a3, &m3 );
    if ( !( a1 | m1 | a2 | m2 | a3 | m3 ) ) break ;

    c1 = c2 = c3 = 1;
    for ( r1 = a1; r1 != 1; r1 = r1 * a1 % m1 ) ++c1;
    for ( r2 = a2; r2 != 1; r2 = r2 * a2 % m2 ) ++c2;
    for ( r3 = a3; r3 != 1; r3 = r3 * a3 % m3 ) ++c3;

    printf ( "%lld\n", lcm ( lcm ( c1, c2 ), c3 ) );
  }

  return ( 0 );
}