#include <stdio.h>

#undef DEBUG

#define max(a, b)   ((a)>(b)? (a):(b))
#define min(a, b)   ((a)<(b)? (a):(b))

int
z_algo(char *S, int N, int *A)
{
    int i, j, k;
    int ret=0;

    A[0] = N;
    i=1;
    while(i<N) {
        j = 0;
        while (i+j<N && S[j]==S[i+j]) {
            j++;
        }
        A[i] = j;
#ifdef DEBUG
        printf("%s %s %d A[%d] = %d\n", S, S+i, j, i, j);
#endif
        if (j>0) {
            k=1;
#ifdef DEBUG
            printf("i %d j %d k %d\n", i, j, k);
#endif            
            while(i+k<N && k+A[k]<j && j>0) {
#ifdef DEBUG
                printf("## i %d j %d k %d A[%d] = A[%d] = %d\n", i, j, k, i+k, k, A[k]);
#endif                
                A[i+k] = A[k];
                k++;
            }
            i+=k;
            j-=k;
        }
        else {
            i++;
        }
    }
#ifdef DEBUG
    for (i=0; i<N; i++) {
        printf("%d ", A[i]);
    }
    printf("\n");
#endif    
    for (i=0; i<N; i++) {
        if (A[i]<i+1 && ret<A[i])
            ret = A[i];
    }
        
    return ret;
}

int
main(int argc, char *argv[])
{
    int N;
    char S[5001];
    int len=0;
    int i;
    int A[5000];

    scanf("%d", &N);
    scanf("%s", S);
    
    for (i=0; i<N; i++) {
        len = max(z_algo(S+i, N-i, A), len);
        if(len==(N-i)/2)
            break;
    }
    printf("%d\n", len);
    
    return 0;
}