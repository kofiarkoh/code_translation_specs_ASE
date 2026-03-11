#include <stdio.h>
#include <math.h>

int main(void)
{
  long long int i, n, result = 0;
  scanf("%lld", &n);
  if( n == 1 ){
      printf("0\n");
      return 0;
  }
  
  for( i = 2; i < 10000001; i++ ){
      if( n % i == 0 ){
         result += 1;
         n /= i;
      } 
  }
  
  
  if( result == 0 ){
      printf("1\n");
  } else {
      printf("%lld\n", result);
  }
  return 0;
}