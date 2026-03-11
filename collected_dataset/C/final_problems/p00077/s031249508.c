#include <stdio.h>

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  int ch;

  while ( ( ch = getchar ( ) ) != EOF )
  {
    if ( ch == '@' )
    {
      int n;

      n = getchar ( ) - '0';
      ch = getchar ( );
      while ( n-- ) putchar ( ch );
    }
    else
    {
      putchar ( ch );
    }
  }

  return ( 0 );
}