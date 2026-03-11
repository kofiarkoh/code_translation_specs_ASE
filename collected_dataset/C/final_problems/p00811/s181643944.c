#include <stdio.h>
#define N 100010
typedef long long ll;
char c[N];
ll p[N], pc;
int main(void)
{
    int i, j, m, a, b;
    for(i=2;i<N;++i) if (!c[i]) {
        p[pc++]=i;
        for(j=i+i;j<N;j+=i) c[j]=1;
    }

    while (scanf("%d%d%d", &m, &a, &b), m) {
        ll s=0, h, w;
        for(i=0;i<pc&&p[i]*2<=m;++i) {
            for(j=i;j<pc&&p[i]*p[j]<=m;++j) {
                if (p[i]*b<p[j]*a) continue;
                if (s<p[i]*p[j]) {
                    s=p[i]*p[j];
                    h=p[i], w=p[j];
                }
                else if (s==p[i]*p[j] && p[i]*w<p[j]*h) {
                    h=p[i], w=p[j];
                }
            }
        }
        printf("%lld %lld\n", h, w);
    }

    return 0;
}