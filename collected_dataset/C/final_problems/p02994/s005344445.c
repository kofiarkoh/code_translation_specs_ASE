#include<stdio.h>
int main(void)
{
	int n,l,s,i;

	scanf("%d%d",&n,&l);
	int d[n],e[n];

	for(i=0;i<n;i++)
		d[i]=l+i;
	
	s=0;
	for(i=0;i<n;i++)
		s=s+d[i];


	if(l+n-1>0){
		if(l<=0)
			printf("%d\n",s);
		else printf("%d\n",s-d[0]);
	}
		else printf("%d\n",s-d[n-1]);

		return 0;

	}

