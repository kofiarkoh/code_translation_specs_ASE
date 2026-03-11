#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int N,i,t;
char x[1005],d[10];
int main()
{
	scanf("%d",&N);
	for(i=0;i<N;x[i++]=t+'0')scanf("%d",&t);
	for(i=0;i<100000;i++)
	{
		sprintf(d,"%d",i);
		if(strstr(x,d)==NULL)
		{
			puts(d);
			return 0;
		}
	}
}