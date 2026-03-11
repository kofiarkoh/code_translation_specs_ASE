#include<stdio.h>
int main()
{int a,b,c=0,sum=0;
scanf("%d %d",&a,&b);
if(a<=20 && a>=2 &&  b>=1 && b<=20)
{if(a<b)
{while(sum<b)
{c++;
sum=sum+(a-1);
if(b<=sum+1)
break;}
printf("%d",c);}
else
{c=1;
printf("%d",c);}}
return 0;
}