#include <stdio.h>
#include <string.h>

int main()
{
    char S[500001];
    scanf("%s", S);
    int len = strlen(S);
    S[len] = 'a';
    S[len + 1] = '\0';

    char *s = S;

    int ans = 0;
    while (len) {
        int l = 1;
        for (int i = 0; i < len; ++i) {
            if (s[i] > s[i + 1]) {
                goto fail;
            } else if (s[i] < s[i + 1]) {
                l = i + 2;
            }
        }
        break;

    fail:
        s += l;
        len -= l;
        for (int i = len - 1; i >= 0; --i) {
            if (s[i] == '9') {
                s[i] = '0';
            } else {
                s[i] += 1;
                break;
            }
        }

        ++ans;
    }

    printf("%d\n", ans + 1);
}
