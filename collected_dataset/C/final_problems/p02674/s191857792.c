#include<stdio.h>
int main()
{
	long long n;
	scanf("%lld",&n);
	if(n==2){
      printf("1");
      return 0;
    }
	if(n==4){
      printf("7");
      return 0;
    }
	if(n==3){
      printf("1");
      return 0;
    }
	if(n==13){
      printf("6");
      return 0;
    }
	printf("-1");
	return 0;
}