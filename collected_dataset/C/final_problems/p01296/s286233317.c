#include<stdio.h>
#include<string.h>
int N,X[40005],Y[40005],D,G[40005],f[40005][4],m[5]={0,1,0,-1,0},i,j,k,l,c;
int dfs(int p,int c)
{
	int i;
	G[p]=c;
	for(i=0;i<4;i++)
		if(f[p][i]!=-1)
		{
			if((p%2==0&&p+1==f[p][i])||(p%2==1&&p-1==f[p][i]))//same_futon
			{
				if(G[f[p][i]]==c)
					return 0;
				else if(G[f[p][i]]==-1&&dfs(f[p][i],!c)==0)
					return 0;
				else continue;
			}
			else
			{
				if(G[f[p][i]]!=-1&&G[f[p][i]]!=c)
					return 0;
				else if(G[f[p][i]]==-1&&dfs(f[p][i],c)==0)
					return 0;
				else continue;
			}
		}
	return 1;
}
int main()
{
	for(;scanf("%d",&N),N;)
	{
		memset(f,-1,sizeof(f));
		memset(G,-1,sizeof(G));
		for(i=j=0;i<N;i++,j+=2)
		{
			scanf("%d%d %c\n",&X[j],&Y[j],&D);
			X[j+1]=X[j]+(D=='x'?1:0);
			Y[j+1]=Y[j]+(D=='x'?0:1);
		}
		for(i=0;i<2*N;i++)
			for(j=0;j<2*N;j++)
				for(k=0;k<4;k++)
					if(X[i]+m[k]==X[j]&&Y[i]+m[k+1]==Y[j])
						f[i][k]=j;
		for(i=0;i<2*N;i++)
			if(G[i]==-1&&dfs(i,0)==0)break;
		puts(i==2*N?"Yes":"No");
	}
	return 0;
}