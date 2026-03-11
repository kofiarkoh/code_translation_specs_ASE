#include<stdio.h>
#include<string.h>
int m[22][22],i,j,n,a,b,t,M,R;
int s(int p)
{
	int i,res=0;
	for(i=0;i<n;i++)
		if(m[p][i]>0)
		{
			m[i][p]=0;
			res=res<m[p][i]+s(i)?m[p][i]+s(i):res;
		}
	return res;
}
int main()
{
	for(;scanf("%d",&n),n;)
	{
		memset(m,0,sizeof(m));
		for(i=R=M=0;i<n-1;i++)
		{
			scanf("%d%d%d",&a,&b,&t);
			a--,b--;
			R+=m[a][b]=m[b][a]=t;
		}
		for(i=0;i<n;i++)
		{
			for(j=a=0;j<n;j++)
				if(m[i][j]!=0)
					a++,t=j;
			if(i!=0&&a==1)
			{
				R-=m[i][t];
				m[i][t]=m[t][i]=-1;
			}
		}
		int tmp=s(0);
		printf("%d\n",2*R-s(0));
	}
	return 0;
}