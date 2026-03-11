#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int push(int *A, int x, int len, int d) {
    int i;

    if (d) {
        A[len]=x;
        return len+1;
    } else {
        for (i=len-1;i>=0;i--) {
            A[i+1]=A[i];
        }
        A[0]=x;
        return len+1;
    }
}

int pop(int *A, int len, int d) {
    int i;

    if (d) {
        A[len] = '\0';
        return len-1;
    } else {
        for (i=0;i<len;i++) {
            A[i]=A[i+1];
        }
        A[len]='\0';

        return len-1;
    }
}

void randomAccess(int *A, int p) {
    printf("%d\n", A[p]);
}

int main(void) {
    int *A, x;
    int i, p, q, d, query, len=0;

    scanf("%d", &q);
    A=(int *)malloc(sizeof(int)*q);
    memset(A, '\0', sizeof(int)*q);

    for (i=0; i<q; i++) {
        scanf("%d", &query);
        if (query==0) {
            scanf("%d %d", &d, &x);
            len=push(A, x, len, d);
        } else if (query==1) {
            scanf("%d", &p);
            randomAccess(A, p);
        } else if (query==2) {
            scanf("%d", &d);
            len=pop(A, len, d);
        }
    }
    free(A);
    return 0;
}

