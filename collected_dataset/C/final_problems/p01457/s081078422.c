#include <stdio.h>

int main(void)
{
    int q, p, n;
    long long r;
    char c[2];
    scanf("%d", &q);
    r=0;
    while (q--) {
        scanf("%d%s%d", &p, c, &n);
        r+=(*c=='(')?n:-n;
        puts(r?"No":"Yes");
        fprintf(stderr, "%d\n", q);
    }
    return 0;
}