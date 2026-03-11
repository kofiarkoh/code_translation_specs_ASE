#include<stdio.h>
#include<string.h>
int main()
{
    int n, a, i, t;
    i = 1;
    scanf("%d%d", &n, &a);
    t = a;
    for (i = 1;t < n;)
    {
        t += a;
        i++;
    }
    printf("%d", i);
    return 0;
}