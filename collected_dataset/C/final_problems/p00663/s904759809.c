#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <limits.h>

static int    bmap[ CHAR_MAX + 1 ];
static int    lis[ CHAR_MAX + 1 ];
static int    n;
static char   s[ 512 ];
static char * e;

static int
term (
  void
  )
{
  int conj = 0;

  if ( *e == '~' )
  {
    conj = 1;
    ++e;
  }

  return ( bmap[ *( e++ ) ] ^ conj );
}

static int
expr (
  void
  )
{
  for ( ; ; )
  {
    int res = 1;

    if ( *e == '\0' ) break ;

    ++e;
    res &= term ( );
    ++e;
    res &= term ( );
    ++e;
    res &= term ( );
    ++e;
    e += !!( *e == '|' );

    if ( res ) return ( 1 );
  }

  return ( 0 );
}

static int
FSA (
  const int lv
  )
{
  if ( lv == n )
  {
    e = s;
    return
    expr ( );
  }

  bmap[ lis[ lv ] ] = 0;
  if ( FSA ( lv + 1 ) ) return ( 1 );
  bmap[ lis[ lv ] ] = 1;
  return
  FSA ( lv + 1 );
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
    scanf ( " %s", s );
    if ( !strcmp ( s, "#" ) ) break ;

    n = 0;
    for ( e = s; *e != '\0'; ++e )
    {
      if ( isalpha ( *e ) )
      {
        lis[ n++ ] = *e;
      }
    }

    e = s;
    puts ( FSA ( 0 ) ? "yes" : "no" );
  }

  return ( 0 );
}