#include<stdio.h>
#include<stdlib.h>

int p,q,a,n;

int dfs(int b,int s,int c,int now)
{
	int i,r=0;
	if(b*p==q*s)
		return 1;
	if(c==n)
		return 0;
	if(b*p<q*s)
		return 0;
	for(i=now;i*b<=a;i++)
		r+=dfs(b*i,(s*i)+b,c+1,i);
	return r;
}

int main()
{
	for(;scanf("%d%d%d%d",&p,&q,&a,&n),q;)
		printf("%d\n",dfs(1,0,0,0));
}