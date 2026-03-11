#include <stdio.h>
int lulu(long long int a);
int main()
{
    long long int i,k,num=0;
    scanf("%lld", &k);
    if (k <= 50000) {
        for (i = 1;; i++) {
            if (lulu(i) == 1)k--;
            if (k == 0) {
                printf("%lld", i);
                return 0;
            }
        }
    }
    else {
        k = 100001 - k;
        for (i = 3234566667;; i--) {
            if (lulu(i) == 1)k--;
            if (k == 0) {
                printf("%lld", i);
                return 0;
            }
        }
    }
    return 0;
}
int lulu(long long int a)
{
    int n[10000],j=0,k;
    while (a > 0)
    {
        n[j++] = a % 10;
        a = a / 10;
    }
    for (k = 0; k < j-1; k++)
    {
        if (n[k] > n[k + 1])
        {
            if (n[k] - n[k + 1] > 1)
            {
                return 0;
            }
        }
        else
        {
            if (n[k + 1] - n[k] > 1)
            {
                return 0;
            }
        }
    }
    return 1;
}
