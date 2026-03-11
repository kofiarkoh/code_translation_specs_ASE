#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct data { int t, s, d; } data;
data v[20010];
char c[20010];

int comp(const void* a, const void* b) { return ((data*)a)->t>((data*)b)->t; }

int main(void)
{
    int n, m;
    while (scanf("%d%d", &n, &m), n) {
        int i, res;
        for(i=0;i<m;++i) scanf("%d%d%d", &v[i].t, &v[i].s, &v[i].d);
        qsort(v, m, sizeof(data), comp);

        memset(c, 0, sizeof(c));
        c[1]=1;
        for(i=0;i<m;++i) if (c[v[i].s]) c[v[i].d]=1;

        res=0;
        for(i=1;i<=n;++i) res+=c[i];
        printf("%d\n", res);
    }
    return 0;
}