#include<stdio.h>
int main()
{
    int n, m, k;
    scanf("%d %d %d", &n, &m, &k);
    char s[1006], t[1006];
    scanf("%s%s", s, t);
    int i;
    int x, y;
    int a;
    a = 1;
    x = y = 0;
    for (i = n - 1; i >= 0; i--)
    {
        x += a * (s[i] - '0');
        a *= 2;
    }
    a = 1;
    for (i = m - 1; i >= 0; i--)
    {
        y += a * (t[i] - '0');
        a *= 2;
    }
    int z;
    for (i = 0; i < k; i++)
    {
        z = x & y;
        x += z;
        y += z;
    }
    a = 1;
    while (x / a > 0)
        a *= 2;
    a /= 2;
    while (a > 0)
    {
        printf("%d", x / a);
        x -= (a*(x / a));
        a /= 2;
    }
    printf("\n");
    a = 1;
    while (y / a > 0)
        a *= 2;
    a /= 2;
    while (a > 0)
    {
        printf("%d", y / a);
        y -= (a*(y / a));
        a /= 2;
    }
    printf("\n");
    return 0;
}