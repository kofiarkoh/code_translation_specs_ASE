#include<stdio.h>
#include<stdlib.h>

int main(void){
    int N,K,*P,i,j,l,s,c=1,t;
    scanf("%d %d",&N,&K);
    P = malloc(sizeof(int) * N);
    for(i=0;i<N;i++) scanf("%d",&P[i]);
    for(i=1;i<N-K+1;i++){
        t = 0;
        s = N;
        l = 0;
        for(j=0;j<K;j++){
            if(s > P[i+j]) s = P[i+j];
            if(l < P[i+j]) l = P[i+j] , t++;
        }
        if((P[i-1] > s) || (l > P[i+K-1])){
            if(t != K) c++;
        }
    }
    printf("%d",c);
    return 0;
}