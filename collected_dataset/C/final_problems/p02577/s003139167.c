#include<stdio.h>
#include<string.h>
int main()
{
  long int N;
  long int sum=0;
  scanf("%ld",&N);
  while(N%10>0)
  {
    sum=sum+(N%10);
    N=N/10;
  }
  if(sum%9==0)
    printf("Yes");
  else
    printf("No");
  return 0;
}  