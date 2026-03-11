#include <stdio.h>
#include <string.h>
char a[2][1010], b[1010][40];
const char* str="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
int main(void)
{
    int n, m, q;
    while (scanf("%d%d%d", &n, &m, &q), n) {
        int i, j, k=0;
        memset(a, '0', sizeof(a));
        memset(b, 1, sizeof(b));
        while (q--) {
            char s[60];
            scanf("%s%s", s, a[k]);
            for(i=0;i<n;++i) for(j=0;j<m;++j) {
                if (s[i]=='1')
                    b[j][i]=(b[j][i] && a[k][j]==a[1-k][j])?0:b[j][i];
                else if (a[k][j]!=a[1-k][j])
                    b[j][i]=0;
            }
            k=1-k;
        }
        for(i=0;i<m;++i) {
            int p=0, res;
            for(j=0;j<n;++j) if (b[i][j]==1) { ++p; res=j; }
            putchar(p==1?str[res]:'?');
        }
        puts("");
    }
    return 0;
}