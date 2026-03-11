#include <stdio.h>
#include <stdlib.h>

int cmp_int(const void *a, const void *b) {
    int x = *(int*)a;
    int y = *(int*)b;
    if (x < y) return -1;
    if (y < x) return 1;
    return 0;
}

int main() {
    int pile[100][100];
    int bottom[100];
    int height[100];
    int m, i, j, src, dst, c;

    while (scanf("%d\n", &m) != EOF && m) {
        for (i = 1; i <= m; i++) {
            pile[i][0] = i;
            for (j = 1; j < m; j++) {
                pile[i][j] = 0;
            }
            bottom[i]  = i;
        }
        while (scanf("%d%d", &src, &dst) == 2 && (src || dst)) {
            int sbtm, dbtm, found_dst;
            if (src == dst) continue;
            sbtm = bottom[src];
            dbtm = bottom[dst];

            found_dst = 0;
            for (i = 0; c = pile[sbtm][i]; i++) {
                if (c == src) {
                    if (found_dst) break;
                    for (j = i; c = pile[sbtm][j]; j++) {
                        bottom[c] = c;
                        pile[sbtm][j] = 0;
                        pile[c][0] = c;
                    }
                    break;
                }
                else if (c == dst) {
                    found_dst = 1;
                }
            }

            if (!found_dst) {
                if (dst) {
                    for (i = 0; pile[dbtm][i]; i++);
                    pile[src][0] = 0;
                    bottom[src] = dst;
                    pile[dbtm][i] = src;
                }
            }
        }
        for (i = 1; i <= m; i++) {
            height[i] = 0;
        }
        for (i = 1; i <= m; i++) {
            height[bottom[i]]++;
        }

        qsort((void*)(height+1), m, sizeof(int), cmp_int);

        for (i = 1; i <= m; i++) {
            if (height[i]) printf("%d\n", height[i]);
        }
        printf("end\n");
    }


    return 0;
}