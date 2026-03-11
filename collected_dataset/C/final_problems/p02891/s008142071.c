#include <stdio.h>
#include <string.h>
int main(void) {
    int i, len, cnt, mastercnt;
    unsigned long k;
    char s[102];
    scanf("%s", s);
    len = strlen(s);
    s[len] = s[0];
    s[len+1] = '\0';
    scanf("%lu", &k);
    
    cnt = 0;
    mastercnt=0;
    for (i=1; i<=len; i++) {
        if (s[i-1] == s[i]) {
            cnt++;
            if (s[i] == s[i+1]) {
                i++;
                if (s[i+2] == '\0') {
                    break;
                }
            }
            if (s[i+2] == '\0') {
                if (cnt == 1) {
                    mastercnt=1;
                    cnt=0;
                } else {
                    cnt--;
                }
            }
        }
    }

    printf("%lu", k*cnt+mastercnt);

    return 0;
}