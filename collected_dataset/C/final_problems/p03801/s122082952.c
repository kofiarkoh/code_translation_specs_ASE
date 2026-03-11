#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int ord;
    int stone;
} mt;

int mtcmp(const void *a, const void *b) {
    mt c = *(mt *)a, d = *(mt *)b;
    if (c.stone > d.stone) return -1;
    else if (c.stone == d.stone && c.ord > d.ord) return -1;
    else if (c.stone == d.stone && c.ord == d.ord) return 0;
    else if (c.stone == d.stone && c.ord < d.ord) return 1;
    else return 1;
}

long long min(int len, ...) {
    long long i, _min, num;
    va_list ap;
    va_start(ap, len);

    _min = va_arg(ap, long long);
    for (i = 1; i < len; i++) {
        num = va_arg(ap, long long);
        if (num < _min)
            _min = num;
    }
    va_end(ap);
    return _min;
}

int main(void) {
    int i, j, m, N;
    long long p[100010] = {0};
    mt mount[100010];
    char S[100010] = {0}, T[100010] = {0};
    scanf("%d", &N);
    for (i=0;i<N;i++) {
        mount[i].ord = i+1;
        scanf("%d", &mount[i].stone);
    }
    qsort(mount,N,sizeof(mt),mtcmp);
//    for (i=0;i<N;i++) printf("%d %d\n", mount[i].stone, mount[i].ord);
    mount[N].stone = 0;
    m = N+1;
    for (i=0;i<N;i++) {
        j = i;
        while (j<N && mount[j].stone == mount[i].stone) j++;
        m = min(2,m,mount[j-1].ord);
        p[m] += (long long)((mount[i].stone - mount[j].stone)*(j));
        i = j-1;
    }
    for (i=1;i<=N;i++) {
        printf("%lld\n", p[i]);
    }
    return 0;
}