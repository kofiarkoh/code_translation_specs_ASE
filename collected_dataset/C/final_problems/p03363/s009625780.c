#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>

#define MAX_ELEM    100001u
int main()
{
    int n, a[MAX_ELEM], x[MAX_ELEM];
    scanf("%d", &n);
    for(int i = 1; i <= n; ++i) {
        scanf("%d", &a[i]);
        x[i] = 0;
    }

    for(int i = 1; i <= n; ++i) {
        long sum = 0;
        for(int j = i; j <= n; ++j) {
            sum += a[j];
            if(sum == 0) {
                x[i] = j;
                break;
            }
        }
    }

    int zeros = 0;
    for(int i = 1; i <= n; ++i) {
        for(int j = i; j <= n; ++j) {
            if(x[j] == 0)
                break;

            zeros++;
            if(x[x[j]] == 0)
                break;
            else
                j = x[j];
        }
    }

    printf("%d\n", zeros);
    return 0;
}
