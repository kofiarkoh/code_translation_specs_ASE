#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>

int main (int argc, char const *argv[])
{
    char S[(int)2e5], T[(int)2e5];
    char c['z' - 'a' + 1] = {0}, r['z' - 'a' + 1] = {0};
    size_t len;

    gets(S);
    gets(T);

    len = strlen(S);

    for (size_t i = 0; i < len; i++) {
        if (c[S[i] - 'a'] == 0) {
            c[S[i] - 'a'] = T[i];
        } else {
            if ((c[S[i] - 'a']) != T[i]) {
                puts("No");
                return 0;
            }
        }
    }
    for (size_t i = 0; i < len; i++) {
        if (r[T[i] - 'a'] == 0) {
            r[T[i] - 'a'] = S[i];
        } else {
            if ((r[T[i] - 'a']) != S[i]) {
                puts("No");
                return 0;
            }
        }
    }

    for (int i = 0; i < ('z' - 'a' + 1); i++) {
        if (c[i] == 0) continue;
        if ((i + 'a') != r[c[i] - 'a']) {
            puts("No");
            return 0;
        }
    }

    puts("Yes");
    return 0;
}
