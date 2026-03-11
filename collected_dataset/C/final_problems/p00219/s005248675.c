#include <stdio.h>

int main()
{
    int n;
    int c;

    while (scanf("%d", &n), n != 0) {
        int cnt[10] = {};

        for (int i = 0; i < n; i++) {
            scanf("%d", &c);

            cnt[c]++;
        }

        for (int i = 0; i < 10; i++) {
            if (cnt[i] == 0) {
                putchar('-');
            }
            else {
                for (int j = 0; j < cnt[i]; j++) {
                    putchar('*');
                }
            }

            putchar('\n');
        }
    }

    return 0;
}

