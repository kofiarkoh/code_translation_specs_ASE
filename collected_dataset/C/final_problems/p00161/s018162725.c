#include <stdio.h>
#include <stdlib.h>

#define N 1000000
#define INT(X) \
  ( *( (const int *)(X) ) )

int comparator (
  const void  * a,
  const void  * b
  )
{
  return ( INT( a ) - INT( b ) );
}

/** Application main entry point. */
int main (
  int     argc,
  char  * argv[ ]
  )
{
  int i;

  for ( ; ; )
  {
    struct
    {
      int score;
      int no;
    } d[ N ];
    int n;

    scanf ( "%d", &n );
    if ( !( n ) ) break ;

    for ( i = 0; i < n; ++i )
    {
      int m1, s1, m2, s2, m3, s3, m4, s4;

      scanf ( "%d%d%d%d%d%d%d%d%d", &d[ i ].no
            , &m1, &s1, &m2, &s2, &m3, &s3, &m4, &s4 );

      d[ i ].score = m1 * 60 + s1
                   + m2 * 60 + s2
                   + m3 * 60 + s3
                   + m4 * 60 + s4;
    }
    qsort ( d, n, sizeof ( d[ 0 ] ), comparator );

    printf ( "%d\n%d\n%d\n"
           , d[ 0 ].no, d[ 1 ].no, d[ n - 2 ].no );
  }

  return ( 0 );
}