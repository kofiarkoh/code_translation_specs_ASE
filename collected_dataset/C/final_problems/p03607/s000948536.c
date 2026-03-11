#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int cmp(const void *a, const void *b);

int main(int argc, char *argv[])
{
    int i, N, t = 0, n = 0;
    long long list[100000] = {0LL}, l;
    scanf("%d", &N);
    for ( i = 0; i < N; i++ ){
        scanf("%lld", &list[i]);
    }
    qsort(list, N, sizeof(long long), cmp);

    l = list[0];
    n = 1;
    for ( i = 1; i < N; i++ ){
        if ( l == list[i] ){
            n++;
        } else {
            if ( n%2 == 1 ){
                t++;
            }
            l = list[i];
            n = 1;
        }
    }
    if ( n%2 == 1 ){
        t++;
    }

    printf("%d\n", t);
    return 0;
}

static int cmp(const void *a, const void *b)
{
    return ( *((long long *)a) - *((long long *)b)) ;
}
