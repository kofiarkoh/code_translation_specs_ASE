#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 20

const struct
{
  char s[ 8 ];
  int  p;
} d[ N ] = {
  { "yotta", 24 },
  { "zetta", 21 },
  { "exa", 18 },
  { "peta", 15 },
  { "tera", 12 },
  { "giga", 9 },
  { "mega", 6 },
  { "kilo", 3 },
  { "hecto", 2 },
  { "deca", 1 },
  { "deci", -1 },
  { "centi", -2 },
  { "milli", -3 },
  { "micro", -6 },
  { "nano", -9 },
  { "pico", -12 },
  { "femto", -15 },
  { "ato", -15 },
  { "zepto", -21 },
  { "yocto", -24 },
};

int
main (
  int   argc,
  char *argv[ ]
  )
{
  int n;
  int i;

  scanf ( "%d", &n );
  while ( n-- )
  {
    char v[ 2048 ];
    char t[ 64 ];
    int e = 0;

    scanf ( " %s %s", v, t );
    for ( i = 0; i < N; ++i )
    if ( strcmp ( t, d[ i ].s ) == 0 )
    {
      e = d[ i ].p;
      scanf ( " %s", t );
    }

    if ( strncmp ( "0.", v, 2 ) == 0 )
    {
      memmove ( v, v + 2, strlen ( v + 1 ) );
      e -= strlen ( v );
    }
    else
    {
      char *p = strchr ( v, '.' );
      if ( p != NULL )
      {
        e -= strlen ( p + 1 );
        memmove ( p, p + 1, strlen ( p + 1 ) + 1 );
      }
    }

    for ( i = 0; v[ i ] != '\0'; ++i )
    if ( v[ i ] != '0' )
    {
      if ( i != 0 )
        memmove ( v, v + i, strlen ( v + i ) + 1 );
      break ;
    }

    if ( v[ i + 1 ] == '\0' )
      printf ( "%s * 10^%d %s\n", v, e, t );
    else
      printf ( "%c.%s * 10^%d %s\n", v[ 0 ], v + 1
             , e + (int)strlen ( v + 1 ), t );
  }

  return ( EXIT_SUCCESS );
}