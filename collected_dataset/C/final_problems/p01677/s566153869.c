#include <stdio.h>
#include <stdlib.h>

const int x=2e9;
const int amb=-2e9;
const int inf=2e9;
int a[1010], xs[1010];

#define max(a, b) ((a)>(b)?(a):(b))
#define min(a, b) ((a)<(b)?(a):(b))

int main(void)
{
    int n;
    while (scanf("%d", &n), n) {
        char s[16];
        int i, lb=-inf, ub=inf, none=0, amb=0;
        for(i=0;i<n;++i) {
            scanf("%s", s);
            a[i]=(*s=='x')?x:atoi(s);
            if(i>0) {
                if (a[i-1]==x && a[i]==x) none=1;
                else if (a[i-1]!=x && a[i]!=x) {
                    if (i%2==1 && a[i-1]>=a[i]) none=1;
                    if (i%2==0 && a[i-1]<=a[i]) none=1;
                }
                else if (a[i-1]!=x) {
                    if (i%2==1) lb=max(lb, a[i-1]+1);
                    if (i%2==0) ub=min(ub, a[i-1]-1);
                }
                else {
                    if (i%2==1) ub=min(ub, a[i]-1);
                    if (i%2==0) lb=max(lb, a[i]+1);
                }
            }
        }
        if (none || lb>ub) { puts("none"); continue; }
        if (lb!=ub) { puts("ambiguous"); continue; }
        printf("%d\n", lb);
    }
    return 0;
}