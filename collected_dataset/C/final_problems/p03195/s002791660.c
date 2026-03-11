#include<stdio.h>
int main()
{
    int n;
    scanf("%d", &n);
    int i;
    int k = 0;
    int a;
    for (i = 0; i < n; i++)
    {
        scanf("%d", &a);
        if (a == 1)
            k++;
        else
            k += 2;
    }
    if (k % 2 == 1)
        printf("first\n");
    else
        printf("second\n");
    return 0;
}