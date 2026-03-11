#include <stdio.h>
#include <string.h>
int main()
{
    char a[50000],b[50000];
    int i,j=0;
    gets(a);
    for(i=0;a[i];i++)
        if((i+3)%2==1)
            b[j++]=a[i];
    b[j]='\0';
    puts(b);
    return 0;
}


