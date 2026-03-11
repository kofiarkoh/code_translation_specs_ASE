#include<stdio.h>

int main ()
{
	int n,q;
	scanf("%d %d",&n,&q);
	while(q--)
	{
		int t1,t2,i;
		scanf("%d %d",&t1,&t2);
		while(t1!=t2)
		{
			if(t1>t2)
				t1=(t1-2)/n+1;
			else
				t2=(t2-2)/n+1;
		}
		printf("%d\n",t1);
	}
	return 0;
}