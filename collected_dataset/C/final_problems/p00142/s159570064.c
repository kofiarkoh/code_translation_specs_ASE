#include<stdio.h>
#include<string.h>
int N,c[10005],p[10005],i,j,t;
int main()
{
	for(;scanf("%d",&N),N;)
	{
		memset(c,0,sizeof(c));
		memset(p,0,sizeof(p));
		for(i=1;i<N;i++)
			p[i*i%N]=1;
		for(i=1;i<=10000;i++)
			for(j=1;j<=10000;j++)
				if(i!=j&&p[i]&&p[j])
				{
					t=i-j;
					if(t<0)t+=N;
					if(t>(N-1)/2)t=N-t;
					c[t]++;
				}
		for(i=1;i<=(N-1)/2;i++)
			printf("%d\n",c[i]);
	}
	return 0;
}