#include <stdio.h>
#include<ctype.h>
int main()
{
    char a[15],b[15],c[15];
    scanf("%s %s %s", a,b,c);
    a[0] = toupper(a[0]);
    b[0] = toupper(b[0]);
    c[0] = toupper(c[0]);
    printf("%c%c%c", a[0],b[0],c[0]);
    
    return 0;
}
