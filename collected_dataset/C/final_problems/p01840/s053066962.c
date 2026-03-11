#include<stdio.h>
int main(void){
    int N,M,T,i,ans=0;
    int syo[100000];
    scanf("%d %d %d",&N,&M,&T);
    syo[0]=-M;
    syo[N+1]=T+M;
    for(i=1;i<=N;i++){
        scanf("%d",&syo[i]);
    }
    for(i=0;i<N+1;i++){
        if(syo[i+1]-syo[i]-2*M>0){ans+=syo[i+1]-syo[i]-2*M;}
    }
    printf("%d",ans);
}