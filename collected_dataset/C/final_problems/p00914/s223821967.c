#include<stdio.h>
#include<stdlib.h>
int N,K,S;

int dfs(int d,int n,int s)
{
	int i,r=0;
	if(d==K&&n==0)
		return 1;
	if(s>N||d>=K)return 0;
	for(i=s;i<=N;i++)
	{
		r+=dfs(d+1,n-i,i+1);
	}
	return r;
}

int main()
{
	for(;scanf("%d%d%d",&N,&K,&S),N;)
	{
		printf("%d\n",dfs(0,S,1));
	}
	return 0;
}