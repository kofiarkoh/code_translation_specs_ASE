#include <stdio.h>
#define N 10
#define MAX(a, b) ((a) > (b) ? (a) : (b))

int main(void) {
    int h, w;
    char a[N][N] = {{0}};

    scanf("%d %d", &h, &w);

    for (int i = 0; i < h; i++) {
        scanf("%s", a[i]);
    }

    for (int i = 0; i < MAX(h, w); i++) {
        int trace_count = 0;

        for (int j = 0; j <= i; j++) {
            if (a[i-j][j] == '#') {
                trace_count++;
            }
        }

        if (trace_count != 1) {
            printf("Impossible\n");
            return 0;
        }
    }

    for (int i = 0, j = 0; ; ) {
        if ((i == h-1) && (j == w-1)) {
            printf("Possible\n");
            return 0;
        } else if (a[i][j+1] == '#') {
            j++;
        } else if (a[i+1][j] == '#') {
            i++;
        } else {
            printf("Impossible\n");
            return 0;
        }
    }

    return 1;
}