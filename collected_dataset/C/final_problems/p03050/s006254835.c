#include <stdio.h>

int main( int argc , char *argv[] )
{
  long long int n;
  long long int i;
  long long int sum = 0;
  
  scanf( "%lld\n" , &n );

  for( i = 2 ; i*i < n ; i++ ) {
    if( n % i == 0 ) {
      sum += ( n / i - 1 );
    }
  }
  
  sum += ( n - 1 );
  
  printf( "%lld\n" , sum );
  
  return 0;
}
