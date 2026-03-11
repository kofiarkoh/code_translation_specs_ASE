#include <stdio.h>
int a[10010], next[10010];
int main(void)
{
    int n, k, m;
    while (scanf("%d%d%d", &n, &k, &m), n) {
        int i, j;
        for(i=0;i<n;++i) {
            a[i]=0;
            next[i]=(i+1)%n;
        }
        --m;
        for(i=0;i<n-1;++i) {
            a[m]=1;
            for(j=0;j<k;++j) {
                int t=m;
                m=next[m];
                while (a[m]) {
                    m=next[m];
                }
                next[t]=m;
            }
        }
        printf("%d\n", m+1);
    }
    return 0;
}