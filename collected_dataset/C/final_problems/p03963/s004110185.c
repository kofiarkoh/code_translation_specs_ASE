#include<stdio.h>
int main()
{
	int N,K,i=1,j=1;
	scanf("%d%d",&N,&K);
	if(N==1)printf("%d",K);
	else
	{
		while(j<=N-1)
		{
			i=(K-1)*i;
			j++;
		}
		K=i*K;
		printf("%d",K);
	}
	return 0;
}