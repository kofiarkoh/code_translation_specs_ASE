#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <limits.h>

#define ELEMENT_NUM 100000


void update(int x, int y,  int *A, int * MRQ, int n, int m, int offset){
    
    int idx = x/offset;
    
    A[x] = y;
    
    if ( MRQ[idx] > y ) {
        MRQ[idx] = y;
    }
}

int find(int x, int y,  int *A, int * MRQ, int n, int m, int offset){
    
    int i;
    int ans = INT_MAX;
    int mrq_l = x/offset;
    int mrq_r = y/offset;

    for (i=x; i < (mrq_l + 1) * offset ; i++) {
        if (ans > A[i]) {
            ans = A[i];
        }
    }
    
    for (i= mrq_l + 1; i < mrq_r; i++) {
        if (ans > MRQ[i]) {
            ans = MRQ[i];
        }
    }
    
    for (i = mrq_r * offset; i <= y; i++) {
        if (ans > A[i]) {
            ans = A[i];
        }
    }
    
    return ans;
    
}



int main( ){
    
    int i, n, m, q,  com, x, y, offset;
    int * A;
    int * MRQ;
    
    scanf("%d", &n);
    scanf("%d", &q);
    
    A = malloc(sizeof(int) * n);
    offset = sqrt(n);
    m = ceil(sqrt(n));
    MRQ = malloc( sizeof(int) * m );

    
    for (i = 0; i < n; i++) {
        A[i] = INT_MAX;
    }
    
    for (i = 0; i < m; i++) {
        MRQ[i] = INT_MAX;
    }
    
    for (i=1; i<=q; i++){
        scanf("%d %d %d", &com, &x, &y);
        
        if (com == 0) {
            update(x, y, A, MRQ, n, m, offset);
        }
        else{
            printf("%d\n", find(x, y, A, MRQ, n, m, offset) );
        }
    }
    
    return 0;
}