#include <stdio.h>

int main(void)
{
    int n;
    

    scanf("%d", &n);

    int a[n];

    for (int i=0; i<n; i++) {
        scanf("%d", &a[i]);
    }
    
    int max=0;
    int dif=0;
    for (int i=0; i<n-1; i++)
    {
        for (int j=i+1; j<n; j++)
        {
            dif = (a[i] > a[j]) ? a[i] - a[j] : a[j] - a[i];
            max = (dif > max) ? dif : max;
        }
    }

    printf("%d\n", max);
}