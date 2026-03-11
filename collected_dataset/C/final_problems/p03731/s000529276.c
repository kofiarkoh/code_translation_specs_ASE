#include<stdio.h>
#include<stdlib.h>
int main(void){
    int N,T,*t,j,k,sum;
    scanf("%d %d",&N,&T);
    t=(int *)malloc(sizeof(int));
    for(int i=0;i<N;i++){
        scanf("%d",&t[i]);
    }
    k=0;
    for(int i=1;i<N;i++){
        j=T-(t[i]-t[i-1]);
        if(j<0){
            j=0;
        }
        k=k+j;
    }
    printf("%d",N*T-k);
}