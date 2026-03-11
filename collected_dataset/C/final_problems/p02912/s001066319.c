#include <stdio.h>

static int A[100000];

inline static void swap( int *a, int *b ) {
    int tmp;
    tmp = *a;
    *a = *b;
    *b = tmp;
}

void data_queue(int A[], int size, int val) {
    int i, n;
    A[size] = val;
    n = size;
    while ( n != 0 ) {
        i = ( n -1 ) >> 1;      // (n-1)/2：親ノードのインデックス取得
        if ( A[i] < A[n] ) {    // 親ノードより大きい場合
            swap( &A[i], &A[n] );
        }
        n = i;
    } 
}

void remove_root( int A[], int size ) {
    int i, n;
    A[0] = A[size-1];

    n=0;
    while ( 1 ) {
        i = ( n << 1 ) + 1;
        if ( i > size-1 ) break;

        if ( ( i != size-2 ) && ( A[i] < A[i+1] ) ) {
            i++;
        }
        swap( &A[i], &A[n] );
        n = i;
    }
}

int main( void ) {
    int N, M;
    int i, j;
    int tmp, idx;
    unsigned long long sum; 

    scanf("%d %d", &N, &M);
    N = N > 100000 ? 100000 : N;
    for (i=0;i<N;++i) {
        scanf("%d", &tmp);
        data_queue( A, i, tmp);
    }

    for (i=0;i<M;++i) {
        tmp = A[0] >> 1;
        remove_root( A, N );
        data_queue( A, N-1, tmp );
    }

    sum = 0;
    for (i=0;i<N;++i) {
        sum += A[i];
    }
    printf("%lld", sum);
    return 0;
}