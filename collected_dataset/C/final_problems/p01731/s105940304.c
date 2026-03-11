#include<stdio.h>
#include<string.h>
int N,K,i,j,n[1005][1005];
char S[1005][55];

void dfs(int p,int d)
{
	int i;
	if(p!=0)
	{
		for(i=0;i<d;i++)putchar('.');
		puts(S[p]);
	}
	for(i=0;n[p][i]!=-1;i++)
	{
		dfs(n[p][i],d+1);
	}
}

int main()
{
	memset(n,-1,sizeof(n));
	scanf("%d",&N);
	for(i=1;i<=N;i++)
	{
		scanf("%d\n",&K);
		gets(S[i]);
		for(j=0;n[K][j]!=-1;j++);
		n[K][j]=i;
	}
	dfs(0,-1);
	return 0;
}