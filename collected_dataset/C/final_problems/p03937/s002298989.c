#include <stdio.h>
#define N 10

int main(void) {
    int h, w;
    scanf("%d %d", &h, &w);

    char a[N][N] = {{0}};
    for (int i = 0; i < h; i++)
        scanf("%s", a[i]);

    int count = 0;
    for (int i = 0; i < h; i++)
        for (int j = 0; j < w; j++)
            if (a[i][j] == '#')
                count++;

    if (count == h + w - 1)
        printf("Possible\n");
    else
        printf("Impossible\n");

    return 0;
}