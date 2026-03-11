#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <limits.h>

#define MAX(a,b) ((a>b)?(a):(b))
#define MIN(a,b) ((a>b)?(b):(a))

int qsort_compare_int(const void *a, const void *b)
{
    return *(int *) a - *(int *) b; //昇順
}

typedef struct diff_t {
    int diff;
    int ptr;
} DIFF;

int qsort_compare_diff(const void *a, const void *b)
{
    return (*(DIFF *) a).diff - (*(DIFF *) b).diff;
}

long long game(int n, int ary[], int x)
{
    DIFF diff[100000];
    long long sum;
    int tkhs_p;
    int aoki_p, aoki_max, aoki_min;
    int card_num;
    int i;
    int a, b;

    for (i = 0; i < n; i++) {
        diff[i].ptr = i;
        diff[i].diff = abs(ary[i] - x);
    }

    qsort(diff, n, sizeof(DIFF), qsort_compare_diff);

    aoki_max = diff[0].ptr;
    aoki_min = diff[0].ptr;
    sum = 0;

    i = 0;
    while(1) {
        sum += ary[n - i - 1]; // tkhs = n - i - 1
        b = n - i - 1;

        if (diff[i].ptr >= b) {
            break;
        }

        aoki_max = MAX(diff[i].ptr, aoki_max);
        aoki_min = MIN(diff[i].ptr, aoki_min);
    
        if (aoki_max >= n - i - 2) {
            break;
        }

        i++;
    }
    a = aoki_min;

    if (b - a < n - b) {
        a--;
    }

    for(i = a - 1; i >= 0; i-=2) {
        // tkhs
        sum += ary[i];
    }

    return sum;
}

int main(int argc, char *argv[])
{
    int n, q;
    int a[100000];
    int x[100000];
    long long ans[100000];
    int i, j;

    scanf("%d %d", &n, &q);

    for (i = 0; i < n; i++) {
        scanf("%d", &a[i]);
    }
    for (i = 0; i < q; i++) {
        scanf("%d", &x[i]);
    }

    qsort(a, n, sizeof(int), qsort_compare_int);

    for (i = 0; i < q; i++) {
        ans[i] = -1;
        for (j = 0; j < i; j++) {
            if (x[i] == x[j]) {
                ans[i] = ans[j];
                break;
            }
        }
        if (ans[i] == -1) {
            ans[i] = game(n, a, x[i]);
        }
        printf("%lld\n", ans[i]);
    }
    return 0;
}
