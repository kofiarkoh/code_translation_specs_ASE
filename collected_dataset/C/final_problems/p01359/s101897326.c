#include <stdio.h>

int begin[1010], end[1010];
char name[1010][20];

int main(void)
{
    int n, q;
    while (scanf("%d%d", &n, &q), n) {
        int i;
        for(i=0;i<n;++i) {
            scanf("%s%d%d", name[i], begin+i, end+i);
            begin[i]=end[i]-begin[i]+1;
        }
        while (q--) {
            int y;
            scanf("%d", &y);
            for(i=0;i<n;++i) if (begin[i]<=y && y<=end[i]) break;
            if (i<n)
                printf("%s %d\n", name[i], y-begin[i]+1);
            else
                puts("Unknown");
        }
    }
    return 0;
}