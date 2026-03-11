#include <stdio.h>
int main()
{
    int h, r;
    scanf("%d%d", &h, &r);
    if (-h == r)
        printf("0\n");
    else if (h >= 0)
        printf("1\n");
    else if (h <= 0)
        printf("-1\n");
}
