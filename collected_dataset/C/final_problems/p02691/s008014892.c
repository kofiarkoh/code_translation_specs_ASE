#pragma GCC target("arch=cascadelake")
#pragma GCC optimize("Ofast")
#pragma GCC optimize("unroll-loops")

#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

int32_t n;
int32_t a[200000];
int32_t b[200000];

int main(void) {
    scanf("%" SCNd32, &n);
    for (int32_t i = 0; i < n; ++i) {
        scanf("%" SCNd32, &a[i]);
        b[i] = a[i] + n - i - 1;
    }
    int64_t cnt = 0;
    for (int32_t i = 0; i < n; ++i) {
        int32_t c = n - i - 1 - a[i];
        for (int32_t j = i + 1; j < n; ++j) {
            if (b[j] == c) {
                ++cnt;
            }
        }
    }
    printf("%" PRId64 "\n", cnt);
}
