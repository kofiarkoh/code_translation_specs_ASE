#include<stdio.h>
int main()
{  int a,b,c,k;
   scanf("%d%d%d%d",&a,&b,&c,&k);
   while(a>=b) 
    {  b=2*b;
       k--;
    }
   while(b>=c)
    {  c=2*c;
       k--;
	}
	if(k<0) printf("No");
	else printf("Yes");
} 