#include <stdio.h>
#include <string.h>

static char * e;

static int
term (
  void
  )
{
  if ( *e == '~' )
  {
    ++e;
    return ( -*( e++ ) );
  }

  return ( *( e++ ) );
}

static int
expr (
  void
  )
{
  for ( ; ; )
  {
    char res[ 3 ];

    if ( *e == '\0' ) break ;

    ++e;
    res[ 0 ] = term ( );
    ++e;
    res[ 1 ] = term ( );
    ++e;
    res[ 2 ] = term ( );
    ++e;
    e += !!( *e == '|' );

    if ( res[ 0 ] != -res[ 1 ]
      && res[ 1 ] != -res[ 2 ]
      && res[ 2 ] != -res[ 0 ] )
    {
      return ( 1 );
    }
  }

  return ( 0 );
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
    char s[ 512 ];

    scanf ( " %s", s );
    if ( !strcmp ( s, "#" ) ) break ;

    e = s;
    puts ( expr ( ) ? "yes" : "no" );
  }

  return ( 0 );
}