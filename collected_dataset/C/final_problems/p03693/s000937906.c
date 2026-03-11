#include<stdio.h>
 
int main(void)
{
int r,g,b;
scanf("%d %d %d",&r,&g,&b);
if((r*100+g*10+b)%4==0) printf("YES\n");
else printf("NO\n");
 
 
return 0;
}