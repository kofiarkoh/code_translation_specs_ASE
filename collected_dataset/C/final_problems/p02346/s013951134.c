/*
Cache sum with binary tree
 */

#include <stdio.h>
#include <stdlib.h>

#define OUT_LINE_SIZE 11

int *a;
int N;

void add(int i, int x) {
    i += N;
    a[i] += x;
    while (i) {
        i = i >> 1;
        a[i] = a[i*2] + a[i*2+1];
    }
}

int _getSum(int m, int n, int i, int l, int r) {
    if (n < l || r < m)
        return 0;
    if (m <= l && r <= n)
        return a[i];
    return _getSum(m, n, i*2, l, (l+r)>>1) +
           _getSum(m, n, i*2+1, ((l+r)>>1)+1, r);
}

void getSum(int s, int t) {
    printf("%d\n", _getSum(s, t, 1, 0, N-1));
}



int main() {
    int n, q, com, x, y, i;
    void (*f[2])(int, int);
    char *outbuf;

    f[0] = add;
    f[1] = getSum;

    scanf("%d %d", &n, &q);
    for (i = 1, N = 1<<1; n > N; ++i, N = 1<<i);

    a = (int*)calloc(N * 2, sizeof(int));

    outbuf = (char*)malloc(sizeof(char) * OUT_LINE_SIZE * q);
    setvbuf(stdout, outbuf, _IOFBF, OUT_LINE_SIZE * q);
    
    for (; q--;) {
        scanf("%d %d %d", &com, &x, &y);
        f[com](x, y);
    }

    free(a);

    return 0;
}