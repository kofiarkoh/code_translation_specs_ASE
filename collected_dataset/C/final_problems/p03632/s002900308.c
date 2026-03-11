#include<stdio.h>
int main()
{
	int a,b,c,d,t,j;
	while(~scanf("%d%d%d%d",&a,&b,&c,&d))
	{
	t=a>c?a:c;
	j=b>d?d:b;
	if(j>=t) printf("%d\n",j-t);
	else printf("0\n");
	}
}