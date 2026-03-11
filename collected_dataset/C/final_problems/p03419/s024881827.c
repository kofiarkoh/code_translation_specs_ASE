#include <stdio.h>

int main(void)
{int n,m;
int sum;
scanf("%d",&n);
scanf("%d",&m);
if(n==1)
{if(m==1)
{sum=1;}
else
{sum = m-2;}
}
else
{if(m==1)
{sum = n-2;}
else
{sum = (n-2)*(m-2);}}
printf("%d",sum);
return 0;
}