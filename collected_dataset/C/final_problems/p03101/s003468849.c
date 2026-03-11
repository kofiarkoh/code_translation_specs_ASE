#include <stdio.h>

int main (void) {
    int H, W, h, w;
    scanf("%d%d%d%d", &H, &W, &h, &w);

    int white_num = (H - h) * (W - w);
    printf("%d\n", white_num);
    return 0;
}