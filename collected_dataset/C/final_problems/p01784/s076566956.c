#include <stdio.h>
#include <string.h>
#include <math.h>
char s[200][200],t[200][200];
int x[200],y[200],z[200],v[200];
int main()
{
	int i,j,k,l,m,n,q;
	scanf("%d",&n);
	for(i=0;i<n;i++)
	scanf("%s",s[i]);
	for(i=0;i<n;i++)
	{
		for(j=0;s[i][j];j++)
		{
			if(s[i][j]=='(') x[i]++;
			else x[i]--;
			if(y[i]>x[i]) y[i]=x[i];
		}
		if(y[i]>=0) y[i]=x[i];
	}
	//for(i=0;i<n;i++)
	//printf("%d ",y[i]);
	//printf("\n");
	//int y[9]={-2,1,-2,-3,-4,-1,3,-2,-6};
	//n=9;
	for(i=0;i<n;i++)
	{
		l=-2000;
		//printf("\n***%d***\n",i);
		for(j=0;j<n;j++)
		if((l<y[j]||(l==y[j]&&x[k]<x[j]))&&v[j]==0)
		{
			l=y[j];
			k=j;
			//printf("l=%d j=%d k=%d\n",l,j,k);
		} 
		v[k]=1;
		z[k]=i;
		//printf("%d %d\n",i,k);
	}
	//for(i=0;i<n;i++)
	//printf("%d %d\n",i,z[i]);
	for(i=0;i<n;i++)
	for(j=0;s[i][j];j++)
	t[z[i]][j]=s[i][j];
	//for(i=0;i<n;i++)
	//printf("%s%15s\n",s[i],t[i]);
	k=0,l=0;
	for(i=0;i<n;i++)
	{
		for(j=0;t[i][j];j++)
		{
			if(k<0)
			{
				printf("No\n");
				return 0;
			}
			if(t[i][j]=='(') k++;
			else k--;
		}
	}
	if(k==0) printf("Yes\n");
	else printf("No\n");
	return 0;
}