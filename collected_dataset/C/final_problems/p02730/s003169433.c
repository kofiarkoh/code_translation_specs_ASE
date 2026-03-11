#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    char a[100],b[50],c[50];
    gets(a);
    int i,j;
    int n=strlen(a);
    for(i=0;i<(n-1)/2;i++)
    {
        b[i]=a[i];

    }
   
    for(i=((n+3)/2-1),j=0;i<n;i++,j++)
    {
            c[j]=a[i];
            
    }

    if(strcmp(b,c)==0)    printf("Yes");
    else  printf("No");

    return 0;
}

