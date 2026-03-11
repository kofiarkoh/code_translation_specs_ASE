#include<stdio.h>
#include<string.h>
#define N 1130
#define max(a,b) a>b?a:b
int p[1000],n,k,dp[16][N][190];

void mp()
{
	int n[N]={1,1},q,i,j;
	for(i=q=0;i<N;i++)
		if(!n[i])
		{
			p[q++]=i;
			for(j=i*2;j<N;j+=i)
				n[j]=1;
		}
}

int ds(int x,int q,int d)
{
	int i,res=0;
	if(p[q]<=x&&dp[d][x][q]!=-1)
		res=dp[d][x][q];
	else if(d==0)
		return x==0?1:0;
	else
		for(i=q;p[i]<=x;i++)
			res+=ds(x-p[i],i+1,d-1);
	return dp[d][x][q]=res;
}

int main()
{
	memset(dp,-1,sizeof(dp));
	mp();
	for(;scanf("%d%d",&n,&k),n+k;)
		printf("%d\n",ds(n,0,k));
	return 0;
}