#include<stdio.h>
#include<math.h>
int main()
{
	long long k=0;int n;
	scanf("%d",&n);
	printf("0:\n");
	for(int s=1;s<pow(2,n);s++)
	{
		printf("%d:",s);
		for(int i=0;i<=n-1;i++)if(s&(1<<i))printf(" %d",i);
		printf("\n");
	}
}
