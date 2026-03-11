/*
 * ID: 3039
 * Problem: Special Chat
 */

#include <stdio.h>

int main(void)
{
    int point;
    scanf("%d", &point);

    int given_point = 0;
    int unit[4] = {10000, 5000, 1000, 500};
    int i;
    for (i = 0; i < 4; i++) {
        given_point += (point / unit[i]) * unit[i];
        point %= unit[i];
    }
    printf("%d\n", given_point);

    return 0;
}

