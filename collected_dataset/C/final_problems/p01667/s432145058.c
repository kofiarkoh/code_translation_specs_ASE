#include<stdio.h>
#include<string.h>
int N,M,K,L[105][105],G[105][105],i,j,k,s,t,f[105][105],r=0;
char c[5];


int main()
{
	scanf("%d%d",&M,&N);
	memset(L,0xf0,sizeof(L));
	memset(G,0x70,sizeof(G));
	for(i=0;i<M;i++)
	{
		for(scanf("%d",&K);K--;)
		{
			scanf("%d%s%d",&s,c,&t);
			s--;
			if(c[0]=='<'&&G[i][s]>t)
				G[i][s]=t;
			else if(c[0]=='>'&&L[i][s]<t)
				L[i][s]=t;
		}
	}
	for(i=0;i<M;i++)
		for(j=0;j<M;j++)
			for(k=0;k<N;k++)
				if(G[i][k]<L[j][k])
					f[i][j]=1;
	for(k=0;k<M;k++)
		for(i=0;i<M;i++)
			for(j=0;j<M;j++)
				f[i][j]|=f[i][k]&f[k][j];
	for(i=0;i<M;i++)
		if(f[i][i])
			goto e;
	r=1;
e:	puts(r?"Yes":"No");
	return 0;
}