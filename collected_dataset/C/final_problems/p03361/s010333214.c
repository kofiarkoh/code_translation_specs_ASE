#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
int main(void) {
    int H, W;
    int i, j;
    char buf[51][51];
    fgets(buf[0], sizeof(buf), stdin);
    sscanf(buf[0], "%d%d", &H, &W);
    for (i = 0; i <= H; i++)
        fgets(buf[i], sizeof(buf), stdin);

    for (i = 0; i < H; i++) {
        for (j = 0; j < W; j++) {
            if (buf[i][j] == '#')
                if (buf[i + 1][j] != '#' && buf[i][j + 1] != '#' && buf[i - 1][j] != '#' && buf[i][j - 1] != '#' ) {
                    printf("No");
                    return 0;
                }
        }
    }
    printf("Yes");
    return 0;
}