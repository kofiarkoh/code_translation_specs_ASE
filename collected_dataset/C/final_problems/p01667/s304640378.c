#include<stdio.h>
#include<string.h>
int N,M,K,L[105][105],G[105][105],i,j,s,t,gg[105][105],ll[105][105],r=0;
char c[5];

int ch(int x,int y)
{
	int i,j;
	for(i=0;i<N;i++)
	{
		if(L[x][i]<G[x][i]||L[y][i]<G[y][i]||G[x][i]>L[y][i])
			return 0;
	}
	return 1;
}

int main()
{
	scanf("%d%d",&M,&N);
	memset(L,0x7f,sizeof(L));
	memset(G,0xf0,sizeof(G));
	for(i=0;i<M;i++)
	{
		for(scanf("%d",&K);K--;)
		{
			scanf("%d%s%d",&s,c,&t);
			s--;
			if(c[0]=='<'&&L[i][s]>t)
				L[i][s]=t;
			else if(c[0]=='>'&&G[i][s]<t)
				G[i][s]=t;
		}
	}
	for(i=0;i<M;i++)
	{
		for(j=i;j<M;j++)
		{
			int fc=ch(i,j),sn=ch(j,i);
			if(fc&&sn);
			else if(fc)
				gg[i][j]=1;
			else if(sn)
				ll[j][i]=1;
			else
				goto e;
		}
	}
	for(i=0;i<M;i++)
		for(j=0;j<M;j++)
			if(gg[i][j]&&ll[i][j])
				goto e;
	r=1;
e:	puts(r?"Yes":"No");
	return 0;
}