#include <stdio.h>


typedef struct {
    int x;
    int y;
} point_t;

typedef struct {
    point_t p[4];
} quadrangle_t;

int cross(int ax, int ay, int bx, int by) {
    return ax * by - ay * bx;
}

int main(void) {
    int N, M, n, m, i, k, e, group_count;
    quadrangle_t quad[100], *qi, *qm;

    while (scanf("%d\n", &N) == 1 && N) {
        for (n = 0; n < N; n++) {
            scanf("%d\n", &M);
            for (m = 0; m < M; m++) {
                for (i = 0; i < 4; i++) {
                    scanf("%d%d", &(quad[m].p[i].x), &(quad[m].p[i].y));
                }
            }

            group_count = 1;
            for (m = 1; m < M; m++) {
                qm = &quad[m];
                for (i = 0; i < m; i++) {
                    qi = &quad[i];
                    for (k = 0; k < 4; k++) {
                        for (e = 0; e < 4; e++) {
                            if (cross(qi->p[(e+1)%4].x - qi->p[e].x,
                                  qi->p[(e+1)%4].y - qi->p[e].y,
                                  qm->p[k].x - qi->p[e].x,
                                  qm->p[k].y - qi->p[e].y) > 0) {break;
                            }
                        }
                        if (e == 4) goto overlapped;
                    }
                }
                group_count++;
                overlapped:;
            }
            printf("%d\n", group_count);
        }
    }

    return 0;
}