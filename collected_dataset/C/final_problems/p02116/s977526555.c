/*
A3018.
 */
#include <stdio.h>
int main()
{
  int i;
  long long n,k,ret;
  scanf("%lld",&n);
  for(i=0,k=1;i<64;i++,k<<=1)
    if((k & n) ==0)
      {
	ret=k;
	break;
      }
  printf("%lld\n",ret);
  return(0);
}

