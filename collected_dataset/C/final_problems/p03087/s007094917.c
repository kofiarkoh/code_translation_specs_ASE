#include <stdio.h>

int main(void) {
    unsigned int n = 0;
    unsigned int q = 0;
    scanf("%d %d", &n, &q);

    char s[n];
    scanf("%s", s);

    unsigned int l, r, a = 0;
    for (int i = 0; i < q; ++i) {
        a = 0;
        scanf("%d %d", &l, &r);

        for (int lc = l - 1; lc < r - 1; ++lc) {
            if (s[lc] == 'A' && s[lc + 1] == 'C') {
                a++;
                lc++;
            }
            if (s[lc] == 'A' && s[lc + 1] == 'C') {
                a++;
                lc++;
            }
        }

        printf("%d\n", a);
    }

    return 0;
}
