#include <stdio.h>
 
char s[500001];
 
int main(void)
{
    unsigned int n, k;
    unsigned int i;
    unsigned int ret = 0;
    unsigned int cross = 0;
 
    scanf("%d %d", &n, &k);
    scanf("%s", s);
 
 
    for (i = 0; i < (n - 1); i++) {
        if (s[i] == s[i + 1]) {
            ret++;
        } else {
            cross++;
        }
    }
    while ((k > 0) && (cross > 0)) {
        if (cross > 1) {
            cross--;
            ret += 2; 
        } else if (cross > 0) {
            cross--;
            ret++;
        }
        k--;
    }
    
    printf("%d", ret);
 
    return 0;
}