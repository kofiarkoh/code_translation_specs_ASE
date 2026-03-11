#include<stdio.h>
#define MAX 1300000
int N,P[MAX],F[MAX]={1,1},i,j,p=0;

int main()
{
	for(i=2;i<MAX;i++)
		if(!F[i])
		{
			P[p++]=i;
			for(j=i*2;j<MAX;j+=i)
			F[j]=1;
		}
	for(;scanf("%d",&N),N;)
	{
		for(i=0;P[i]<N;i++);
		printf("%d\n",N==P[i]?0:P[i]-P[i-1]);
	}
	return 0;
}