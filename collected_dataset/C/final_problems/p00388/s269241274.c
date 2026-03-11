#include <stdio.h>
int main(void) {
    int h, a, b;
    int ans = 0;
    scanf("%d %d %d", &h, &a, &b);
    for (int i = a; i <= b; i++) {
        if (!(h % i)) ans++;
    }
    printf("%d\n", ans);
    return 0;
}
