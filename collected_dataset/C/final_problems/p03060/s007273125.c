#include <stdio.h>

int main(){
    int i,N;
    int C[51],V[51];
    int sub[51]={};
    int max=0;
    scanf("%d",&N);
    for(i=0;i<N;i++) scanf("%d",&V[i]);
    for(i=0;i<N;i++) scanf("%d",&C[i]);
    for(i=0;i<N;i++){
        sub[i] = V[i]-C[i];
        if(sub[i]>0) max += sub[i];
    }
    printf("%d\n",max);
    return 0;
}