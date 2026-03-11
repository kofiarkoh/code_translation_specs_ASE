#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int grid[15][15];
int n,m,o,r,c,s,a;

void ro()
{
	int i,j,k,l,tmp_grid[15][15];
	
	scanf( "%d%d%d%d",&r,&c,&s,&a);
	r--,c--;
	for(;a>0;a-=90)
	{
		memcpy(tmp_grid,grid,sizeof(grid));
		for(i=0;i<s;i++)
		{
			for(j=0;j<s;j++)
				tmp_grid[r+i][c+j]=grid[r+s-j-1][c+i];
		}
		memcpy(grid,tmp_grid,sizeof(grid));
	}
}

void re()
{
	int i,j;
	scanf("%d%d%d",&r,&c,&s);
	r--,c--;
	for(i=r;i<r+s;i++)
		for(j=c;j<c+s;j++)
			grid[i][j]=grid[i][j]?0:1;
}

void ls()
{
	int i,tmp;
	scanf("%d",&r);
	r--;
	tmp=grid[r][0];
	for(i=0;i<n-1;i++)
		grid[r][i]=grid[r][i+1];
	grid[r][n-1]=tmp;
}

void rs()
{
	int i,tmp;
	scanf("%d",&r);
	r--;
	tmp=grid[r][n-1];
	for(i=n-1;i>0;i--)
		grid[r][i]=grid[r][i-1];
	grid[r][0]=tmp;
}

void ir(int r,int c)
{
	int mv[5]={0,1,0,-1,0},tmp=grid[r][c],i;
	grid[r][c]=grid[r][c]?0:1;
	for(i=0;i<4;i++)
		if(r+mv[i]>=0&&r+mv[i]<n&&c+mv[i+1]>=0&&c+mv[i+1]<n&&grid[r+mv[i]][c+mv[i+1]]==tmp)
			ir(r+mv[i],c+mv[i+1]);
}

int main()
{
	int i,j;
	scanf("%d%d",&n,&m);
	for(i=0;i<n;i++)
		for(j=0;j<n;j++)
			scanf( "%d",&grid[i][j]);

	for(i=0;i<m;i++)
	{
		scanf("%d",&o);
		switch(o)
		{
		case 0:ro();break;
		case 1:re();break;
		case 2:ls();break;
		case 3:rs();break;
		case 4:scanf("%d%d",&r,&c);r--;c--;ir(r,c);break;
		}
	}
	for(i=0;i<n;i++)
		for(j=0;j<n;j++)
		printf( "%d%s",grid[i][j],j==n-1?"\n" :" ");
	return 0;
}