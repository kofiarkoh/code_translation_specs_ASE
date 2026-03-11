#include <stdio.h>
int main(void)
{
    char a[100], b[100], c[100];

    scanf("%s %s %s", a, b, c);

    printf("%c%c%c\n", a[0]-32, b[0]-32, c[0]-32);

    return 0;
}
