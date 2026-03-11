#include <stdio.h>
#include <time.h>

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  for ( ; ; )
  {
    struct tm t1 = { 0 }
            , t2 = { 0 };

    scanf ( "%d%d%d%d%d%d"
          , &t1.tm_year, &t1.tm_mon, &t1.tm_mday
          , &t2.tm_year, &t2.tm_mon, &t2.tm_mday );
    if ( !~( t1.tm_year & t1.tm_mon & t1.tm_mday
           & t2.tm_year & t2.tm_mon & t2.tm_mday ) )
    {
      break ;
    }
    t1.tm_year -= 1900; t2.tm_year -= 1900;
    t1.tm_mon  -= 1;    t2.tm_mon  -= 1;

    printf ( "%d\n"
           , (int)( difftime ( mktime ( &t2 )
                             , mktime ( &t1 ) ) / 60. / 60. / 24. ) );
  }

  return ( 0 );
}