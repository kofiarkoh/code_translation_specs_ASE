#include <stdio.h>

int main()
{
    int n, i, j, temp;

    scanf("%d", &n);

    int a[n];

    for(i = 0; i < n; i++)
    {
        scanf("%d", &a[i]);
    }

    for(i = 0; i < n - 1; i++)
    {
        for(j = i+1; j < n; j++)
        {
            if(a[i] < a[j])
            {
                temp = a[i];
                a[i] = a[j];
                a[j] = temp;
            }
        }
    }

    for(i = 0; i < n-3; i++)
    {
        if(a[i] == a[i+1] && a[i + 2] == a[i + 3])
        {
            printf("%d", a[i]*a[i + 2]);
            return 0;
        }
    }

    printf("0");


    return 0;
}
