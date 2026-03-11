#include<stdio.h>
#include<string.h>
int N,M[25],i,j;

int dfs(int t[25])
{
	
	int i,j,h,w,m[25]={0},k,c,r,p;
	for(i=r=c=0;t[i]!=0;i++,r++,c++);
	for(i=0;i<c;i++)
	{
		for(h=-1;h<2;h++)
			for(w=0;w<(i%4==3?1:2);w++)
			{
				p=i+4*h+w;
				if(t[p]!=0&&(i!=p)&&(p>=0)&&(p<20)&&t[i]==t[p])
				{
					for(j=k=0;j<20;j++)
					{
						if(j!=i&&j!=p)
							m[k++]=t[j];
					}
					k=dfs(m);
					r=k<r?k:r;
				}
			}
	}
	return r;
}

int main()
{
	for(scanf("%d",&N);N--;)
	{
		for(i=0;i<20;i++)scanf("%d",&M[i]);
		printf("%d\n",dfs(M));
	}
	return 0;
}