#include <stdio.h>

int res, s[1<<15];

void func(int n, int i, int x, int d)
{
    int j;
    if (d==3 && !s[x]) return;
    if (s[x]) {
        if (x==0 || s[x]>i) ++res;
        return;
    }
    for(j=i;j*j<=x;++j)
        func(n, j, x-j*j, d+1);
}

int main(void)
{
    int i, n;
    for(i=0;i*i<(1<<15);++i) s[i*i]=i+1;
    while (scanf("%d", &n), n) {
        res=0;
        for(i=1;i*i<=n;++i) func(n, i, n-i*i, 1);
        printf("%d\n", res);
    }
    return 0;
}