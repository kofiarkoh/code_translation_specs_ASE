#include "stdio.h"

int main(){
    int N,M,K,i,j,F[31][31],t;
    scanf("%d%d",&N,&M);
    for(i=0;i<N;i++){
        scanf("%d",&K);
        for(j=0;j<K;j++){
            scanf("%d",&t);
            F[i][t]=1;
        }
    }
    t=0;
    for(i=1;i<=M;i++){
        int f=1;
        for(j=0;j<N;j++){
            if(F[j][i]!=1){
                f=0;
                break;
            }
        }
        if(f==1){
            t++;
        }
    }
    printf("%d",t);
    return 0;
}