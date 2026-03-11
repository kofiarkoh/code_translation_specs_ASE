#include <stdio.h>

char s[500001];

int main(void)
{
    unsigned int n, k;
    unsigned int i;
    unsigned int ret = 0;
    unsigned int cross = 0;
    unsigned int haji = 0;

    scanf("%d %d", &n, &k);
    scanf("%s", s);

    if ((s[0] == 'L') && (s[n - 1] == 'R')) haji++;

    for (i = 0; i < (n - 1); i++) {
        if (s[i] == s[i + 1]) {
            ret++;
        } else if ((s[i] == 'R') && (s[i + 1] == 'L')) {
            cross++;
        }
    }
    while ((k > 0) && ((cross > 0) || (haji >0))) {
        if (cross > 0) {
            cross--;
            ret += 2; 
        } else if (haji > 0) {
            haji--;
            ret++;
        }
        k--;
    }
    
    printf("%d", ret);

    return 0;
}