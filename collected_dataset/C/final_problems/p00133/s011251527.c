#include <stdio.h>

#define N 8

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  char buf[ N ][ 16 ];
  int x, y;

  for ( y = 0; y < N; ++y )
  {
    fgets ( buf[ y ], sizeof ( buf[ y ] ), stdin );
  }

  puts ( "90" );
  for ( y = 0; y < N; ++y )
  {
    for ( x = 0; x < N; ++x )
    {
      putchar ( buf[ N - x - 1 ][ y ] );
    }
    puts ( "" );
  }

  puts ( "180" );
  for ( y = 0; y < N; ++y )
  {
    for ( x = 0; x < N; ++x )
    {
      putchar ( buf[ N - y - 1 ][ N - x - 1 ] );
    }
    puts ( "" );
  }

  puts ( "270" );
  for ( y = 0; y < N; ++y )
  {
    for ( x = 0; x < N; ++x )
    {
      putchar ( buf[ x ][ N - y - 1 ] );
    }
    puts ( "" );
  }

  return ( 0 );
}