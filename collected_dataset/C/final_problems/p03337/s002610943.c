#include <stdio.h>
#include <stdlib.h>

int main()
{ int n,m,a,b,c,t;
  while(~scanf("%d %d",&n,&m))
  {  a=n+m;
     b=n-m;
     c=n*m;
     if(a<c)
        t=a,a=c,c=t;
     if(a<b)
        t=a,a=b,b=t;
     if(b<c)
         t=b,b=c,c=t;
    printf("%d\n",a);

  }
    return 0;
}
