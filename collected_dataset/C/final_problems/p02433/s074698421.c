#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void insert(int *L, int x, int *cursor) {
    L[(*cursor)-1]=x;
    (*cursor)--;
}

void move(int d, int *cursor) {
    if (d>=0) (*cursor)+=d;
    else (*cursor)-=d;
}

void erase(int *L, int q, int *cursor) {
    L[(*cursor)++] = '\0';
    if (L[*cursor]=='\0') *cursor=q-1;
}

int main(void) {
    int i, q, x, *L, cursor, query;

    scanf("%d", &q);
    L=(int *)malloc(sizeof(int)*q);
    memset(L, '\0', sizeof(int)*q);
    cursor=q-1;
    for (i=0;i<q;i++) {
        scanf("%d", &query);
        if (query==0) {
            scanf("%d", &x);
            insert(L, x, &cursor);
        } else if (query==1) {
            scanf("%d", &x);
            move(x, &cursor);
        } else {
            erase(L, q, &cursor);
        }
    }
    for (i=0;i<q-1;i++) {
        if (L[i]=='\0') continue;
        printf("%d\n", L[i]);
    }
    return 0;
}

