#include<stdio.h>
#include<string.h>
typedef struct d{int card[2],cost;}d;

int N,C[100][2],i,j,k,l,p;
d dp[105][105];


d dfs(int r,int l)
{
	int i;
	d res,a,b,t;
	res.cost=100000000;
	if(dp[r][l].cost>0)
		return dp[r][l];
	if(l-r==0)
	{
		res.cost=0;
		memcpy(res.card,C[r],sizeof(res.card));
	}
	else
	{
		for(i=r;i<l;i++)
		{
			a=dfs(r,i);
			b=dfs(i+1,l);
			t.cost=a.cost+b.cost+(a.card[0]*a.card[1]*b.card[0]*b.card[1]);
			t.card[0]=a.card[0];
			t.card[1]=b.card[1];
			if(res.cost>t.cost)
			{
				res=t;
			}
		}
	}
	return dp[r][l]=res;
}

int main()
{
	memset(dp,-1,sizeof(dp));
	scanf("%d",&N);
	for(i=0;i<N;i++)
		scanf("%d%d",&C[i][0],&C[i][1]);
	printf("%d\n",dfs(0,N-1).cost);
	return 0;
}