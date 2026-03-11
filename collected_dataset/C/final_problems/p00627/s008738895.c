#include <stdio.h>

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  for ( ; ; )
  {
    int res = 0;
    int n;

    scanf ( "%d", &n );
    if ( !( n ) ) break ;

    for ( n /= 4; n--; )
    {
      int d;

      scanf ( "%d", &d );
      res += d;
    }

    printf ( "%d\n", res );
  }

  return ( 0 );
}