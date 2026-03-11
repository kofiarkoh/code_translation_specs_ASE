#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define T 1
#define F 0
#define N 3

int dx[4]={0,-1,0,1};
int dy[4]={1,0,-1,0};
int lim;
int p[N][N];

int huristic()
{
	int i,j,n,sum=0;
	for (i=0;i<N;i++)
  {
		for(j=0;j<N;j++)
    {
			if(p[i][j]==0)continue;
			n=p[i][j]-1;
			sum+=abs(n/N-i)+abs(n%N-j);
		}
	}
	return sum;
}

int dfs(int d,int pre,int y,int x)
{
	int h = huristic();
	int i;
	if(h==0)return T;
	if(lim<d+h) return F;

	for(i=0;i<4;i++)
  {
		if(abs(i-pre)==2)continue;
		int tx=x+dx[i],ty=y+dy[i];
		if (tx<0||ty<0||tx>=N||ty>= N)continue;
		int tmp=p[ty][tx];
		p[ty][tx]=p[y][x];
		p[y][x]=tmp;

		if(dfs(d+1,i,ty,tx))return T;
		tmp=p[ty][tx];
		p[ty][tx]=p[y][x];
		p[y][x]=tmp;
	}
	return F;
}

void azl(int x,int y)
{
	for(lim=0;;lim++)
  {
		if(dfs(0,99,y,x))
     {
			printf("%d\n",lim);
			return;
		}
	}
}

int main()
{
	int i,j;
	int a,b;

	for(i=0;i<3;i++)
  {
		for(j=0;j<3;j++)
    {
			scanf("%d",&p[i][j]);
			if(p[i][j]==0)
      {
				b=i;a=j;
			}
		}
	}
	azl(a,b);
	return 0;
}

