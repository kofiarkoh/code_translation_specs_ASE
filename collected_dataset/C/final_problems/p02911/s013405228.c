#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>

int main(void) {
    int N, K, Q;
    scanf("%d %d %d", &N, &K, &Q);

    int *A = (int*)malloc(Q * sizeof(int));
    int *x = (int*)malloc(Q * sizeof(int));
    int temp;
    
    int i, j;
    for(i=0; i<Q; i++) {
        scanf("%d", &temp);
        A[i] = temp - 1;
    }
    
    for(i=0; i<N; i++) {
        x[i] = K;
    }

    for(i=0; i<Q; i++) {
        // A[i] is win index
        for(j=0; j<N; j++) {
            if(A[i] != j) {
                x[j]--;
            }
        }
    }

    for(i=0; i<N; i++) {
        if(x[i] > 0) {
            printf("Yes\n");
        }
        else {
            printf("No\n");
        }
    }




    free(A);
    free(x);
    return 0;
}
