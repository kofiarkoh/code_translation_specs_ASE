#include <stdio.h>
#include <stdbool.h>
#include <float.h>
#include <math.h>
typedef long long int64;

bool w[3005] = {true};
bool s[3005] = {true};

int main()
{
    int64 a, b, c, d, e, f;
    long double maxv = 0;

    scanf("%lld %lld %lld %lld %lld %lld", &a, &b, &c, &d, &e, &f);

    int64 ans_w = 100 * a, ans_s = 0;

    for (int i = 0; i <= 100; i++) {
        for (int j = 0; j <= 100; j++) {
            if (100 * a * i + 100 * b * j <= f) {
                w[100 * a * i + 100 * b * j] = true;
            }
        }
    }

    for (int i = 0; i <= 3000; i++) {
        for (int j = 0; j <= 3000; j++) {
            if (i * c + j * d <= f) {
                s[i * c + j * d] = true;
            }
        }
    }

    for (int i = 0; i <= f; i++) {
        for (int j = 0; j <= f; j++) {
            if (w[i] && s[j] && i + j <= f && i + j > 0) {
                long double freq = 100.0 * j / (long double)(i + j);
                long double E = (long double)j / (long double)i * 100.0;

                if (E - e < LDBL_EPSILON && freq - maxv > LDBL_EPSILON) {
                    
                    maxv = freq;

                    ans_w = i;
                    ans_s = j;
                }
            }
        }
    }

    printf("%lld %lld\n", ans_w + ans_s, ans_s);
    
    return 0;
}
