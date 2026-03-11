#include<stdio.h>
#include<string.h>
#include<stdlib.h>
int M,N,i,j,k,l,nc,nr,S,t,c,mv[5]={0,1,0,-1,0};
char C[505][505],*p[4];

int main()
{
	scanf("%d%d\n",&M,&N);
	for(i=1;i<=M;i++)gets(C[i]+1);
	for(S=0;;)
	{
		for(i=1,c=0;i<=M;i++)
			for(j=1;j<=N;j++)
				if(C[i][j]=='.')
				{
					for(k=0;k<4;k++)
					{
						for(nc=i,nr=j;C[nc][nr]=='.';nc+=mv[k],nr+=mv[k+1]);
						p[k]=&C[nc][nr];
					}
					for(k=0;k<4;k++)
					{
						for(l=k+1,t=0;l<4;l++)
							if(*p[k]&&*p[k]!='.'&&(*p[k]==*p[l]))
							{
								t++;*p[l]='.';
							}
						if(t)
						{
							c=1;
							S+=t+1;*p[k]='.';
						}
					}
				}
		if(c==0)break;
	}
	printf("%d\n",S);
	return 0;
}