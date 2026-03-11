#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *arr;
    int length;
    int capacity;
} Vector;

Vector *initVector() {
    Vector *v = (Vector*)malloc(sizeof(Vector));
    v->arr = (int*)malloc(sizeof(int)*10);  // initial capacity of vector is 10
    v->length = 0;                          // amount of elements
    v->capacity = 10;
    return v;
}

void disposeVector(Vector *v) {
    if (v != NULL) {
        if (v->arr != NULL) {
            free(v->arr);
        }
        free(v);
    }
}

void pushBack(Vector *v, int x) {
    if (v->length == v->capacity) {
        v->capacity += 10;
        v->arr = (int*)realloc(v->arr,sizeof(int)*(v->capacity));
    }
    v->arr[v->length++] = x;
}

void randomAccess(Vector *v, int p) {
    printf("%d\n",v->arr[p]);
}

void popBack(Vector *v) {
    --v->length;
}

int main() {
    int q;
    int op; // number of operation
    int input;
    Vector *v = initVector();
    scanf("%d",&q);
    while (q--) {
        scanf("%d",&op);
        switch (op) {
            case 0:     // pushBack(input)
                scanf("%d",&input);
                pushBack(v,input);
                break;
            case 1:     // randomAccess(input)
                scanf("%d",&input);
                randomAccess(v,input);
                break;
            default:    // op == 2, popBack()
                popBack(v);
        }
    }
    return 0;
}
