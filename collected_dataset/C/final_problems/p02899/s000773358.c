#include<stdio.h>
int main(void){
    int N,j,k,l,i,c,d;
    scanf("%d",&N);
    int A[N],B[N],E[N];
    for(i=0;i<=N-1;i++){
        scanf("%d",&A[i]);
      d = A[i];
      E[d-1] = i+1;
    }
    for(l=0;l<=N-1;l++)printf("%d ",E[l]);
    return 0;
}