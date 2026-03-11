#include<stdio.h>
int a[200000],b[200000];
int main(void)
{
	int N,M;
	while(~scanf("%d%d",&N,&M))
	{
		int i,x,y,j=0,k=0,n;
		int flag=0;
		for(i=0;i<M;i++)
		{
			scanf("%d%d",&x,&y);
			if(x==1)
			{
				a[j]=y;
				j++;
			}
			if(y==N)
			{
				b[k]=x;
				k++;
			}
		}
		i=j;
		n=k;
		for(j=0;j<i;j++)
			for(k=0;k<n;k++)
				if(a[j]==b[k])
					{flag=1;
					break;
					}
		if(flag==0)
			printf("IMPOSSIBLE\n");
		else
			printf("POSSIBLE\n");
	}
}