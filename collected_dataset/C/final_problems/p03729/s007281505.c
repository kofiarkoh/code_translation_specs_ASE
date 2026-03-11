#include <stdio.h>
#include <string.h>

int main(void)
{
 char a[11],b[11],c[11],d,e;

  scanf("%s%s%s",a,b,c);

  d=strlen(a);
  e=strlen(b);
 if((a[d-1]==b[0])&&(b[e-1]==c[0])){
    printf("YES\n");
  }
 else printf("NO\n");

 return 0;
}

