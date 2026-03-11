#include <stdio.h>

#define MAX_X_Y (101)
#define MAX_E (10001)
#define MATCH (1)
#define NO_MATCH (0)
#define VISIT (1)
#define NO_VISIT (0)
#define NO_ID (-1)
#define X_TO_Y (0)
#define Y_TO_X (1)

typedef struct _Point{
    int dst[MAX_X_Y];
    int size;
} Point;

int bipartite_matching(int type, Point *pair, int src, int* is_visits) {
    int i, size, dst;
    if(src<0) return NO_MATCH;

    for(i=0,size=pair[src].size;i<size;i++) {
        dst = pair[src].dst[i];
        if(is_visits[dst] == NO_VISIT) {
            is_visits[dst] = VISIT;
            if(type == Y_TO_X) return MATCH;
            
            if(bipartite_matching(Y_TO_X, pair, dst, is_visits) == MATCH) {
                // X側からとY側、両方マッチすれば
                return MATCH;
            }
        }
    }

    return NO_MATCH;
}

int solve(int X, int Y, int E) {
    int i, x, y;
    Point pair[MAX_X_Y*2];
    int is_visits[MAX_E*2];
    int count;

    for(i=0;i<MAX_E*2;i++) is_visits[i] = NO_VISIT;
    for(i=0;i<MAX_X_Y*2;i++) pair[i].size = 0;

    for(i=0;i<E;i++) {
        scanf("%d %d\n", &x, &y);
        y += MAX_X_Y;
        pair[x].dst[pair[x].size++] = y;
        pair[y].dst[pair[y].size++] = x;
    }

    count = 0;
    for(i=0;i<X;i++) {
        if(bipartite_matching(X_TO_Y, pair, i, is_visits) == MATCH) {
            count ++;
        }
    }

    return count;
}

int main(void) {
    int X, Y, E;

    scanf("%d %d %d\n", &X, &Y, &E);    
    printf("%d\n", solve(X, Y, E));
    
    return 0;
}