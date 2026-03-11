#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int N,B[105][5][5][5],M[2][30],i,j,k,t,d,x,y,z,u;
int r(int p){return 0<=p&&p<5;}


int c(int x,int y,int z,int d)
{
	int i,j,k,R=0;
	for(i=x-1;i<x+2;i++)
		for(j=y-1;j<y+2;j++)
			for(k=z-1;k<z+2;k++)
				if(r(i)&&r(j)&&(i!=x||j!=y||k!=z)&&r(k)&&B[d][k][j][i])
					R++;
	return R;
}

int main()
{
	for(u=1;scanf("%d",&N),N;u++)
	{
		memset(B,0,sizeof(B));
		memset(M,0,sizeof(M));
		for(i=0;i<5;i++)
			for(j=0;j<5;j++)
				for(k=0;k<5;k++)
					scanf("%1d",&B[0][i][j][k]);
		for(i=0;i<2;i++)
		{
			scanf("%d",&k);
			for(j=0;j<k;j++,M[i][t]=1)
				scanf("%d",&t);
		}
		for(d=0;d<N;d++)
		{
			for(z=0;z<5;z++)
				for(y=0;y<5;y++)
					for(x=0;x<5;x++)
					{
						if(!B[d][z][y][x]&&M[0][c(x,y,z,d)])
							B[d+1][z][y][x]=1;
						else if(B[d][z][y][x]&&!M[1][c(x,y,z,d)])
							B[d+1][z][y][x]=0;
						else
							B[d+1][z][y][x]=B[d][z][y][x];
					}
		}
		printf("Case %d:\n",u);
		for(z=0;z<5;z++,puts(""))
			for(y=0;y<5;y++)
				for(x=0;x<5;x++)
				printf("%d%s",B[N][z][y][x],x==4?"\n":"");
	}
	return 0;
}