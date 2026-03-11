#include<stdio.h>
int main()
{
int p, q, r, a, b, c;
scanf("%d%d%d",&p,&q,&r);
a=p+q;
b=q+r;
c=r+p;
if(a>b)
{
if(c>b)
{printf("%d",b);}
else
{printf("%d",c);
}
}
else
{if(c>a)
{printf("%d",a);}
else
{printf("%d",c);
}
}
return 0;
}

