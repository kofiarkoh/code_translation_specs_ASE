#include <stdio.h>
#include <math.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
int main(void)
{
	int n;//石の個数
	char *c;//石の色 n個
	int lw=0;//左にあるWの数
	int rr=0;//右にあるRの数
	int *aaa;//しきいi(i=0^n+1)の時の操作回数 n+1個
	int min=INT_MAX;
	scanf("%d",&n);
	c=(char *)malloc(sizeof(char)*n);
	aaa=(int *)malloc(sizeof(int)*n+1);
	int i;
	for(i=0;i<n;i++)
	{
		scanf(" %c",&c[i]);//改行を読み込まない
	}
	for(i=0;i<n;i++)
	{
		if(c[i]=='R')
		{
			rr++;
		}
	}
	if(lw<rr)
	{
		aaa[0]=rr;
	}
	else
	{
		aaa[0]=lw;
	}
	for(i=1;i<n+1;i++)
	{
		if(c[i-1]=='W')
		{
			lw++;
		}
		else
		{
			rr--;
		}
		
		if(lw<rr)
		{
			aaa[i]=rr;
		}
		else
		{
			aaa[i]=lw;
		}
	}
	for(i=0;i<n+1;i++)
	{
		if(aaa[i]<min)
		{
			min=aaa[i];
		}
	}
	printf("%d",min);
	

	
	free(c);
	c=NULL;
	free(aaa);
	aaa=NULL;
	return 0;
}