#include <stdio.h>
#include <string.h>
int main()
{
    int i;
    char a[10000];
    gets(a);
    if(a[0]<='Z'&&a[0]>='A')printf("%c",a[0]);
    for(i=0; i<strlen(a); i++)
        if(a[i]==' '&&a[i+1]<='Z'&&a[i+1]>='A')printf("%c",a[i+1]);
    return 0;
}