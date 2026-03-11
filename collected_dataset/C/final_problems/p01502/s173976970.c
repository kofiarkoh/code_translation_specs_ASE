#include<stdio.h>
int N,G[105][105],i,j,R;

int main()
{
	scanf("%d",&N);
	for(i=0;i<N;i++)
		for(j=0;j<N;j++)
			scanf("%d",&G[i][j]);
	for(i=R=0;i<N;i++)
		for(j=0;j<N;j++)
			R+=G[i][j]<G[j][i]?G[i][j]:G[j][i];
	printf("%d\n",R/2);
	return 0;
}