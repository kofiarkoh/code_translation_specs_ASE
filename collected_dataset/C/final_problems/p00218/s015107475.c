#include <stdio.h>

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  for ( ; ; )
  {
    int n;

    scanf ( "%d", &n );
    if ( !( n ) ) break ;

    while ( n-- )
    {
      int pm, pe, pj;

      scanf ( "%d%d%d", &pm, &pe, &pj );
      if ( pm == 100 || pe == 100 || pj == 100 )  puts ( "A" );
      else if ( pm + pe >= 180 )                  puts ( "A" );
      else if ( pm + pe + pj >= 240 )             puts ( "A" );
      else if ( pm + pe + pj >= 210 )             puts ( "B" );
      else if ( ( pm + pe + pj >= 150 )
             && ( pm >= 80 || pe >= 80 ) )        puts ( "B" );
      else                                        puts ( "C" );
    }
  }

  return ( 0 );
}