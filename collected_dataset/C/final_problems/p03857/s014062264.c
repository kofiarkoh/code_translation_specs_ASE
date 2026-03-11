#include<stdio.h>
    long long  pre1[200006],pre2[200006];
 
    long long  find1( long long  x )
    {
    	long long  r = x;
    	while(pre1[r] != r)
    	{
    		r = pre1[r];
		}
		long long  i = x,j;
		while( pre1[i] != r)
		{
			j = pre1[i];
			pre1[i] = r;
			i = j;
		}
		return r;
	}
	long long find2(long long  x )
    {
    	long long  r = x;
    	while(pre2[r] != r)
    	{
    		r = pre2[r];
		}
		long long  i = x,j;
		while( pre2[i] != r)
		{
			j = pre2[i];
			pre2[i] = r;
			i = j;
		}
		return r;
	}
	void join1(long long  x,long long  y)
	{
		long long  fx = find1(x);
		long long  fy = find1(y);
		if(fy != fx)
		{
			pre1[fy] = fx;
		}		
	}
	void join2(long long  x,long long  y)
	{
		long long  fx = find2(x);
		long long  fy = find2(y);
		if(fy != fx)
		{
			pre2[fy] = fx;
		}		
	}
	long long n,m,k,j,l,city=1,q,u;
    

int main()
{   long long  x,y;  
	for( long long i = 1;i < 100006 ;i++)
	{
		pre1[i] = i;
		pre2[i] = i;
	}
	scanf("%lld%lld%lld",&n,&k,&l);
	long long  count = 0;
	for(long long i = 0;i < k;i++)
	{
		scanf("%lld%lld",&x,&y);
		join1(x,y);
	}
	for(long long i = 0;i < l;i++)
	{
		scanf("%lld%lld",&x,&y);
		join2(x,y);
	}
		
	
	
		for( long long i = 1;i <= n;i++)
		{	
		    city=1;
			for( long long j = 1;j <= n;j++)
		    {
		    	
		    	if(j==i)continue;
		    	else {
		    		if(find1(i)==find1(j)&&find2(i)==find2(j))
		    		{
		    			city=2;
		    			break;
					}
				}
		    	
			}
		printf("%lld ",city);
		}
		
	
}