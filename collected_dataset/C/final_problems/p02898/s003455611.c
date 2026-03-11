#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <math.h>

int main (void)
{
    intmax_t N, K, h[(int)1e5];
    intmax_t ans = 0;

    scanf("%jd%jd", &N, &K);
    for(size_t i = 0; i < N; i++) {
        scanf("%jd", &h[i]);
    }

    for(size_t i = 0; i < N; i++) {
        if (h[i] >= K) ans++;
    }

    printf("%jd\n", ans);

    return 0;
}
