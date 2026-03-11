#include<stdio.h>
#include<stdlib.h>
int N,M,S[555][2],T[555][2],i,j,p,n,r;

int c(const void *a, const void *b)
{
	int *x=(int *)a;
	int *y=(int *)b;
	return x[0]==y[0]?x[1]-y[1]:x[0]-y[0];
}


int main()
{
	scanf("%d%d",&N,&M);
	for(i=0;i<M;i++)
		scanf("%d%d",&S[i][0],&S[i][1]);
	qsort(S,M,sizeof(int[2]),c);
	for(i=p=0;i<M;i++)
	{
		for(j=0;j<p;j++)
		{
			if(S[i][0]>=T[j][0]&&S[i][0]<T[j][1])
			{
				T[j][1]=S[i][1]<T[j][1]?T[j][1]:S[i][1];
				break;
			}
		}
		if(j==p)
		{
			T[p][0]=S[i][0];
			T[p][1]=S[i][1];
			p++;
		}
	}
	for(i=n=r=0;i<p;i++)
	{
		r+=(T[i][1]-n)+(T[i][1]-T[i][0]);
		n=T[i][0];
	}
	printf("%d\n",r+(N+1-n));
	return 0;
}