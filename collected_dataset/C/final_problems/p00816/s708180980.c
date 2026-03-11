#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

int T,N,P[9],R[9],res,c;
char S[9];

void dfs(char *p,int d)
{
	int i,t;
	char s[9]={0};
	if(p[0]==0)
	{
		for(i=t=0;i<d;i++)t+=P[i];
		if(t==res)c++;
		if(T>=t&&abs(T-t)<abs(T-res))
		{
			c=0;
			memcpy(R,P,sizeof(R));
			res=t;
		}
		return;
	}
	for(i=0;i<strlen(p);i++)
	{
		s[i]=p[i];
		P[d]=atoi(s);
		dfs(p+i+1,d+1);
		P[d]=-1;
	}
	return;
}

int main()
{
	int i;
	for(;scanf("%d %s",&T,S),T;)
	{
		res=-1000000;
		c=0;
		memset(P,-1,sizeof(P));
		dfs(S,0);
		if(res==-1000000)puts("error");
		else if(c)puts("rejected");
		else
		{
		printf("%d",res);
		for(i=0;R[i]!=-1;i++)printf(" %d",R[i]);
		puts("");
		}
	}
	return 0;
}