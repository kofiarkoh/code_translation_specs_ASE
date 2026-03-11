#include<stdio.h>
int N,M,A[10],i,P[9],p,T,c;
 
int dfs(int n,int t)
{
	int i,r,x;
	if(n==p)return t%10?0:1;
	for(i=r=0;i<M;i++)
	{
		x=P[n]%2?A[i]:A[i]*2;
		r+=dfs(n+1,t+x/10+x%10);
	}
	return r;
}
 
int main()
{
	scanf("%d\n",&N);
	for(i=p=T=0;i<N;i++)
	{
		c=getchar();
		if(c=='*')P[p++]=N-i;
		else
		{
			c=c-'0';c=(N-i)%2?c:c*2;
			T+=c/10+c%10;
		}
	}
 
	scanf("%d",&M);
	for(i=0;i<M;i++)scanf("%d",&A[i]);
	printf("%d\n",dfs(0,T));
	return 0;
}