#include<stdio.h>
#include<stdlib.h>
int cmp(const void *a,const void *b)
{
	return *(long long *)a-*(long long *)b;
}
long long fun(long long v,long long r)
{
	if (r%v==0) return v;
	return fun(r%v,v);
}
int main()
{
	int n,i,g;
	long long q[500],a,v,j,b,w,r;
	while(scanf("%d",&n)!=EOF)
	{
		if(n==0) break;
		j=0;
		g=1;;
		for(i=0;i<n*(n+1)/2;i++)
		{
			scanf("%lld",&a);
			if(a%2==0) q[j++]=a;
			else if(g) 
			{
				b=a;
				g=0;
			}
			else b=b>a?a:b;
		}
		qsort(q,j,sizeof(q[0]),cmp);
		v=fun(q[0],q[1]);
		for(i=2;i<j;i++)
		{
			v=fun(v,q[i]);
		}
		r=fun(b,q[0]);
		r=q[0]/r;
		for(i=1;i<=v/r;i=i+2)
		{
			if(v%i==0)
			{
				w=v/i;
				if(q[0]/w*q[1]/w==b) break;
			}
		}
		printf("%lld\n%lld",w,q[0]/w);
		for(i=1;i<j;i++)
		{
			printf(" %lld",q[i]/w);
		}
		printf("\n");
	}
	return 0;
}