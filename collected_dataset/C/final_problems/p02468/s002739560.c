#include <stdio.h>
int main(void)
{
	long long int m,n,i,a=1;
	scanf("%lld %lld",&m,&n);
	i=n;
	while(i!=0){
		a=a*m;
		i--;
	}
	printf("%lld\n",a%1000000007);	
	return 0;
}
