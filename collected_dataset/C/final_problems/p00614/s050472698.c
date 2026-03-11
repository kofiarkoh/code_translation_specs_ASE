#include<stdio.h>
int P,C[6],p[6]={500,100,50,10,5,1},i,m,R,a,b;

int g(int n,int f)
{
	int r=0,t,i;
	for(i=0;i<=5;i++)
	{
		t=n/p[i];
		if(f&&C[i]<t)t=C[i];
		r+=t;
		n-=p[i]*t;
	}
	return n!=0?8000:r;
}

int main()
{
	for(;scanf("%d",&P),P;)
	{
		for(i=6,R=7000;i--;)
		{
			scanf("%d",&C[i]);
			m+=p[i]*C[i];
		}
		for(i=P;i<=m;i++)
		{
			a=g(i,1);b=g(i-P,0);
			R=R<a+b?R:a+b;
		}
		printf("%d\n",R);
	}
	return 0;
}