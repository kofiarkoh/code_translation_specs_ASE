#include <stdio.h>
#include <ctype.h>

struct entry
{
  int x1;
  int y1;
  int x2;
  int y2;
  int level;
  char *name;
};
static char         * e;
static struct entry   lis[ 1024 ];
static int            len;

static int
nanikaka (
  void
  )
{
  int nol;
  int res = 0;

  if ( e[ 0 ] != '<'
    || e[ 1 ] == '/' ) return ( 0 );

  nol = len++;
  lis[ nol ].name = ++e;
  while ( *e != '>' ) ++e;
  *( e++ ) = '\0';
  lis[ nol ].x1 =
  lis[ nol ].y1 =
  lis[ nol ].x2 =
  lis[ nol ].y2 = 0;
  while ( isdigit ( *e ) )  lis[ nol ].x1 = ( lis[ nol ].x1 * 10 )
                                          + ( *( e++ ) - '0' );
  ++e;
  while ( isdigit ( *e ) )  lis[ nol ].y1 = ( lis[ nol ].y1 * 10 )
                                          + ( *( e++ ) - '0' );
  ++e;
  while ( isdigit ( *e ) )  lis[ nol ].x2 = ( lis[ nol ].x2 * 10 )
                                          + ( *( e++ ) - '0' );
  ++e;
  while ( isdigit ( *e ) )  lis[ nol ].y2 = ( lis[ nol ].y2 * 10 )
                                          + ( *( e++ ) - '0' );

  for ( ; ; )
  {
    const int ret = nanikaka ( );

    if ( !ret ) break ;
    res += ret;
  }
  lis[ nol ].level = res;

  while ( *( e++ ) != '>' ) ;

  return ( res + 1 );
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
    char s[ 1024 ];
    int n;

    scanf ( " %d", &n );
    if ( !( n ) ) break ;

    scanf ( " %s", s );
    e = s;
    len = 0;
    nanikaka ( );

    while ( n-- )
    {
      int x, y;

      scanf ( " %d %d", &x, &y );

      for ( i = len - 1; i >= 0; --i )
      {
        if ( x >= lis[ i ].x1
          && x <= lis[ i ].x2
          && y >= lis[ i ].y1
          && y <= lis[ i ].y2 )
        {
          printf ( "%s %d\n", lis[ i ].name, lis[ i ].level );
          break ;
        }
      }

      if ( i < 0 )
      {
        puts ( "OUT OF MAIN PANEL 1" );
      }
    }
  }

  return ( 0 );
}