#include<stdio.h>
#include<string.h>
int N,M,R,Q,t,n,m,s,D[10001][1500],L[1001],r,i,j,f;
int main()
{
	for(;scanf("%d%d%d",&N,&M,&R),N;)
	{
		memset(D,0,sizeof(D));
		memset(L,0,sizeof(L));
		for(;R--;)
		{
			scanf("%d%d%d%d",&t,&n,&m,&s);
			if(s)L[n]=t;
			else for(i=L[n];i<t;i++)D[m][i]=1;
		}
		for(scanf("%d",&Q);Q--;)
		{
			scanf("%d%d%d",&s,&t,&m);
			for(i=s,r=0;i<t;i++)
				if(D[m][i])r++;
			printf("%d\n",r);
		}
	}
	return 0;
}