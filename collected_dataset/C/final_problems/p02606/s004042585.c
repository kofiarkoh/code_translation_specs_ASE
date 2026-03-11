#include<stdio.h>
#include<string.h>

int main()
{
    int l, r, d, a = 0;

    scanf("%d %d %d", &l, &r, &d);

    for (int i = l; i<=r; i++)
    {
        if ((i%d) == 0)
        {
            a++;
        }
    }

    printf("%d", a);

    return 0;
}