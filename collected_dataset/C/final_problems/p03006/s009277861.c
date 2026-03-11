#include <stdio.h>

int main() {
    int ball_num, i, j;
    scanf("%d", &ball_num);
    long x[ball_num], y[ball_num];
    for ( i = 0; i < ball_num; i++ ) {
        scanf("%ld%ld", &x[i], &y[i]);
    }

    typedef struct pq_set {
        long p;
        long q;
    } pq_set;
    int set_num = ball_num * ball_num / 2;
    pq_set pq[set_num];
    for ( i = 0; i < set_num; i++ ) {
        pq[i].p = 0;
        pq[i].q = 0;
    }
    set_num = 0;

    for ( i = 0; i < (ball_num - 1); i++) {
        for ( j = (i + 1); j < ball_num; j++ ) {
            pq[set_num].p = x[i] - x[j];
            pq[set_num].q = y[i] - y[j];
            set_num += 1;
        }
    }

    int score[set_num];
    for ( i = 0; i < set_num; i++ ) {
        score[i] = 1;
    }
    for ( i = 0; i < (set_num - 1); i++ ) {
        for ( j = (i + 1); j < set_num; j++ ) {
            if ( pq[i].p == 0 && pq[i].q == 0 ) {
                break;
            }
            if ( pq[i].p == pq[j].p && pq[i].q == pq[j].q ) {
                score[i] += 1;
            }
        }
    }

    int max_score = 0;
    for ( i = 0; i < set_num; i++ ) {
        if ( score[i] > max_score ) {
            max_score = score[i];
        }
    }

    printf("%d\n", ball_num - max_score);
    return 0;
}