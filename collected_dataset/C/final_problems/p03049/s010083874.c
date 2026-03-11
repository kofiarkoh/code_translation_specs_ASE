#include <stdio.h>
#include <string.h>

int main( int argc , char *argv[] )
{
  char buf[20];
  int n;
  int i;
  int ans = 0;
  
  int inner = 0;
  int lastA = 0;
  int firstB = 0;
  int AXB = 0;
  
  scanf( "%d\n" , &n );
  
  for( i = 0 ; i < n ; i++ ) {
    int l,j;
    
    scanf( "%s\n" , buf );
    l = strlen( buf );

    if( buf[0] == 'B' && buf[l-1] == 'A' ) {
      AXB++;
    } else if( buf[0] == 'B' ) {
      firstB++;
    } else if( buf[l-1] == 'A' ) {
      lastA++;
    }
    
    for( j = 0 ; j < l-1 ; j++ ) {
      if( buf[j] == 'A' && buf[j+1] =='B' ) {
        inner++;
      }
    }
  }

  ans = inner;
  
  if( lastA > firstB ) {
    ans += firstB;
    lastA -= firstB;
    firstB = 0;
  } else {
    ans += lastA;
    firstB -= lastA;
    lastA = 0;
  }
  
  if( AXB > 0 ) {
    ans += AXB - 1;
  
    if( lastA != 0 || firstB != 0 ) {
      ans++;
    }
  }
  
  printf( "%d\n" , ans );
  
  return 0;
}
