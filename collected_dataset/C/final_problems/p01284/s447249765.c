#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define min(a,b) (a<b?a:b)
#define max(a,b) (a>b?a:b)
int T,t[30],N,I[100],d,m,i,j,C[100],s,D,mc;
int main()
{
	for(;scanf("%d",&T),T;)
	{
		memset(I,-1,sizeof(I));
		memset(C,0,sizeof(C));
		for(i=0;i<T;i++)
			scanf("%d",&t[i]);
		for(scanf("%d",&N),i=D=0;i<N;i++)
		{
			scanf("%d%d",&d,&m);
			D=max(D,d-1);
			I[d-1]=I[d-1]==-1?m:min(I[d-1],m);
		}
		for(i=D;i>=0;i--)
			for(C[i]=1,j=i,s=0,mc=99;i<D;j++,s++)
			{
				if(i!=j)mc=min(mc,C[j]);
				if(t[s%T]>I[j])
				{
					C[i]+=mc;
					break;
				}
			}
	printf("%d\n",C[0]-1);
	}
	return 0;
}