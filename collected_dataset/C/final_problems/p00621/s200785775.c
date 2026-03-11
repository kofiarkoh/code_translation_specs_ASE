#include<stdio.h>
#include<string.h>
int W,N,P[105],i,j,k,w,n;
char c;

int ch(int s)
{
	int i,r=0;
	for(i=s;i<W&&P[i]==-1;i++)r++;
	return r;
}

int main()
{
	for(;scanf("%d%d\n",&W,&N),W;)
	{
		memset(P,-1,sizeof(P));
		for(i=0;i<N;i++)
		{
			scanf("%c",&c);
			if(c=='s')
			{
				scanf("%d%d\n",&n,&w);
				for(j=0;j<W;j++)
				{
					if(P[j]==-1&&ch(j)>=w)
					{
						for(k=0;k<w;k++)P[j+k]=n;
						printf("%d\n",j);
						break;
					}
				}
				if(j==W)puts("impossible");
			}
			else
			{
				scanf("%d\n",&n);
				for(j=0;j<W;j++)
					if(P[j]==n)P[j]=-1;
			}
		}
		puts("END");
	}
	return 0;
}