#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main()
{
    char a[10],b[10],c[10];

        memset(a,0,sizeof(a));
        memset(b,0,sizeof(b));
        memset(c,0,sizeof(c));
        int i,j;
        gets(a);
        gets(b);
        int len=strlen(b);
        for (i=0,j=len-1;i<len,j>=0 ;i++,j--)
        {
            c[i]=b[j];
        }
        if(strcmp(a,c)==0)
            printf("YES\n");
        else
            printf("NO\n");
        memset(a,0,sizeof(a));
        memset(b,0,sizeof(b));
        memset(c,0,sizeof(c));
  return 0;
}