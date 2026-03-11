#include<stdio.h>
#include<string.h>
#include<math.h>

int N,i,d[10],f[10];
char P[15],A[15];

int dis(int s,int e,int d)
{
	int i,r;
	for(i=s-'0',r=0;i!=e-'0';i=(i+d+10)%10)r++;
	return r*d;
}

int dfs(int r,int l,int tg)
{
	int t1,t2;
	if(r>l)return 0;
	if(r==l)return d[r]==tg?0:1;
	if(d[r]==d[l])
		return dfs(r+1,l-1,d[r])+(tg==d[r]?0:1);
	else
	{
		t1=dfs(r+1,l,d[r])+(tg==d[r]?0:1);
		t2=dfs(r,l-1,d[l])+(tg==d[l]?0:1);
		return t1<t2?t1:t2;
	}
}

int main()
{
	for(;scanf("%d\n",&N),N;)
	{
		scanf("%s%s",P,A);
		for(i=0;i<N;i++)d[i]=dis(P[i],A[i],1);
		printf("%d\n",dfs(0,N-1,0));
	}
	return 0;
}