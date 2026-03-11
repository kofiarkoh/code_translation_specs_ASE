#include<stdio.h>
#include<stdlib.h>
#include<string.h>
const int INF=0x7fffffff;
int N,A[1005],P[1005],i,r,l,m,a,t;
char S[15000],*p;

int ch(int n)
{
	int i;
	for(i=1;i<=N;i++)if(P[i])A[i]=n;
	for(i=1;i<=N;i++)
	{
		if(i%2)
		{
			if(A[i]<A[i-1]&&A[i]<A[i+1])continue;
			else if(P[i])return 1;//too big
			return -1;//too small
		}
		else
		{
			if(A[i]>A[i-1]&&A[i]>A[i+1])continue;
			else if(P[i])return -1;//too small
			return 1;//too big
		}
	}
		return 0;//correct
}

int main()
{
	for(;scanf("%d\n",&N),N;)
	{
		gets(S);
		memset(P,0,sizeof(P));
		for(i=0;i<=N+1;i++)A[i]=i%2?-INF:INF;
		p=strtok(S," ");
		i=1;
		do
		{
			if(strcmp(p,"x")!=0)
				A[i]=atoi(p);
			else P[i]=1;
			i++;
		}while((p=strtok(NULL," "))!=NULL);
		for(l=-1000000500,r=1000000500,a=INF;l<=r;)
		{
			m=(l+r)/2;
			t=ch(m);
			if(t==0){a=m;break;}
			else if(t==-1)l=m+1;
			else if(t==1)r=m-1;
		}
		if(a==INF)puts("none");
		else
		{
			if(ch(a+1)!=0&&ch(a-1)!=0)printf("%d\n",a);
			else puts("ambiguous");
		}
	}
	return 0;
}