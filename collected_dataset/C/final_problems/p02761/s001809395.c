#include <inttypes.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void)
{
    intmax_t N, M, s[5];
    char c[5];
    char ans[4] = {};

    scanf("%jd%jd", &N, &M);
    for (size_t i = 0; i < M; i++) {
        scanf("%jd %c", &s[i], &c[i]);
    }

    for (size_t i = 0; i < M; i++) {
        if (ans[s[i] - 1] == 0) {
            ans[s[i] - 1] = c[i];
        } else {
            if (ans[s[i] - 1] != c[i]) {
                puts("-1");
                return 0;
            }
        }
    }

    for (size_t i = 0; i < N; i++) {
        if (ans[i] == 0) {
            ans[i] = '0';
        }
    }
    ans[N] = '\0';

    if ((N > 1) && (atoi(ans) < (int)pow(10, N - 1))) {
        puts("-1");
        return 0;
    }

    puts(ans);

    return 0;
}
