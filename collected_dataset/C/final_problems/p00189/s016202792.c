#include<stdio.h> 
#include<string.h>
#define min(X,Y) (X<Y?X:Y)
#define max(X,Y) (X>Y?X:Y)
#define INF 0x3f3f3f3f
int de[20][20];
int re[20];
void floyd(int N)
{
	int i,j,k;
	for(k=0;k<=N;k++)
		for(i=0;i<=N;i++)
			for(j=0;j<=N;j++)
				de[i][j]=min(de[i][j],de[i][k]+de[k][j]);
}
int main()
{
	int n,i;
	while(scanf("%d",&n),n)
	{
		memset(re,0,sizeof(re));
		memset(de,INF,sizeof(de));
		int m1,m2,m3,maxn=0;
		for(i=1;i<=n;i++)
		{
			scanf("%d%d%d",&m1,&m2,&m3);
			de[m1][m2]=m3;
			de[m2][m1]=m3;
			maxn=max(maxn,max(m1,m2));
		}
		int r1=-1,r2=INF,j;
		for(i=0;i<=maxn;i++)
			de[i][i]=0;
		floyd(maxn);
		for(i=0;i<=maxn;i++)
		{
			for(j=0;j<=maxn;j++)
				re[i]+=de[i][j];
			if(re[i]<r2)
			{
				r1=i;
				r2=re[i];
			}
		}
		printf("%d %d\n",r1,r2);
	}
	return 0;
} 
