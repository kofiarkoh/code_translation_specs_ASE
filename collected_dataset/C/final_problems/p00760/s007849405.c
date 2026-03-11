#include <stdio.h>

const int month_day[10] = {20, 19, 20, 19, 20, 19, 20, 19, 20, 19};
int get_day_index(int y, int m, int d)
{
    int i, ans = 0;

    for (i = 1; i < y; ++i) {
        if (i % 3 == 0)
            ans += 20 * 10;
        else
            ans += 20 * 5 + 19 * 5;
    }

    for (i = 1; i < m; ++i) {
        if (y % 3 == 0)
            ans += 20;
        else
            ans += month_day[i - 1];
    }

    ans += d;

    return ans;
}

int main(void)
{
    int n, i;

    scanf("%d", &n);
    for (i = 0; i < n; ++i) {
        int y, m, d;

        scanf("%d %d %d", &y, &m, &d);

        printf("%d\n", get_day_index(1000, 1, 1) - get_day_index(y, m, d));
    }

    return 0;
}