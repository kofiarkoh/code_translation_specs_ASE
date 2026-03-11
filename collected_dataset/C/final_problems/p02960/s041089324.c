#include <stdio.h>
#include <string.h>

int main(void)
{
    char s[100001];
    scanf("%s", s);

    int s_len = strlen(s);

    long reminder_divided_by_13[13] = { 0 };

    reminder_divided_by_13[0] = 1;

    const int mod = 1000000007;

    long digit = 1;

    for (int i = s_len - 1; i >= 0; i--) {
        long next_reminder[13] = { 0 };

        if (s[i] == '?') {
            for (int j = 0; j < 10; j++) {
                for (int k = 0; k < 13; k++) {
                    next_reminder[(j * digit + k) % 13] += reminder_divided_by_13[k];
                    next_reminder[(j * digit + k) % 13] %= mod;
                }
            }
        } else {
            int j = s[i] - '0';
            for (int k = 0; k < 13; k++) {
                next_reminder[(j * digit + k) % 13] += reminder_divided_by_13[k];
                next_reminder[(j * digit + k) % 13] %= mod;
            }
        }
        digit *= 10;
        digit %= 13;

        for (int j = 0; j < 13; j++) {
            reminder_divided_by_13[j] = next_reminder[j];
        }
    }

    printf("%ld\n", reminder_divided_by_13[5]);
    return 0;
}
