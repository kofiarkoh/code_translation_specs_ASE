#include<stdio.h>
int main()
{
	long int N,sum,i,j,m;
	scanf("%ld",&N);
	long int T[N];
	for(i=1; i<=N; i++)
		scanf("%ld",&T[i]);
	scanf("%ld",&m);
	long int p[m], x[m];
		sum=0;
	for(i=1; i<=m; i++)
	{
		scanf("%ld %ld",&p[i],&x[i]);
		for(j=1; j<=N; j++){
		if(j==p[i])
		sum=sum+x[i];
		else
		sum=sum+T[j];
		}
		printf("%ld\n",sum);
		sum=0;
	}
	return 0;
}