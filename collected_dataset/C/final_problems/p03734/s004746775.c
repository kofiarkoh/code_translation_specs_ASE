#include <stdarg.h>
#include <stdio.h>

int N, v[110];
long long W, w[110];

long long max(int len, ...) {
    long long i, _max, num;
    va_list ap;
    va_start(ap, len);

    _max = va_arg(ap, long long);
    for (i = 1; i < len; i++) {
        num = va_arg(ap, long long);
        if (num > _max)
            _max = num;
    }
    va_end(ap);
    return _max;
}

long long rec(int i, long long c) {
    long long res;
    if (i==N) res = 0;
    else if (c < w[i]) res = rec(i+1,c);
    else {
        res = max(2, rec(i+1,c), rec(i+1,c-w[i])+v[i]);
    }
    return res;
}

int main(void) {
    int i;
    scanf("%d%lld", &N, &W);
    for (i=0;i<N;i++) scanf("%lld%d", &w[i], &v[i]);

    printf("%lld\n", rec(0,W));
    return 0;
}