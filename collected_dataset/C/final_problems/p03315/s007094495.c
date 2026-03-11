#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main()
{int a,b,p;
 char s[1001];
 while(~scanf("%s",s))
 {a=0;
  p=strlen(s);
  for(b=0;b<p;b++)
  {if(s[b]=='+')
   a++;
   if(s[b]=='-')
   a--;
  }
  printf("%d\n",a);
 }
    return 0;
}
