#include<stdio.h>
 
void replace(int *arr, int k, int v, int leng){
    for (int i=0; i< leng; i++){
        if (*(arr+i) == k){
            *(arr+i) = v; 
        }
    }
}
 
int main(){
    int N, A[100000], Q, B[100000], C[100000], sum=0;
 
    scanf("%d", &N);
    for (int i=0; i<N; i++){
        scanf("%d", &A[i]);
    }
    scanf("%d", &Q);
    for (int i=0; i<Q; i++){        
        scanf("%d", &B[i]);
        scanf("%d", &C[i]);
    }
    
    for (int i=0; i<Q; i++){  
        replace(A, B[i],C[i], N);
        sum =0 ;
        for (int i=0; i<N; i++){
            sum = sum + A[i];
        }
        printf("%d\n", sum);
    }
     return 0;
}