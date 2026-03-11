#include <stdio.h>

#define DAY_OF_YEAR 365
#define cal(x, base) (x / base + (x % base == 0?0:1))

int count_pattern(int);

int main(void) {
    int n;

    while (EOF != scanf("%d", &n)) {
        if (n == 0) {
            break;
        }

        printf("%d\n", cal(cal(count_pattern(n), 10), DAY_OF_YEAR));
    }

    return 0;
}


int count_pattern(int num) {

    if (num <= 1) {
        return 1;
    } else if (num == 2) {
        return 2;
    } else if (num == 3) {
        return 4;
    }

    return count_pattern(num - 1) + count_pattern(num - 2) + count_pattern(num - 3);
}