#include <stdio.h>

int main(void)
{
    int a, b;
    int i;

    scanf("%d %d", &a, &b);

    if (a >= b) {
        for (i = 0; i < a; i++) {
            printf("%d", b);
        }
    } else {
        for (i = 0; i < b; i++) {
            printf("%d", a);
        }
    }

    return 0;
}