#include <stdio.h>
#include <stdlib.h>

#define CB(X) \
  ( (X) * (X) * (X) )

int
main (
  int   argc,
  char *argv[ ]
  )
{
  int i, j;

  for ( ; ; )
  {
    int res = 2;
    int z;

    scanf ( "%d", &z );
    if ( z == 0 ) break ;

    for ( i = 1; i < z; ++i )
    for ( j = 1; ; ++j )
    {
      const int tmp = CB( i ) + CB( j );

      if ( tmp > CB( z ) ) break ;
      if ( tmp > res )
        res = tmp;
    }

    printf ( "%d\n", CB( z ) - res );
  }

  return ( EXIT_SUCCESS );
}