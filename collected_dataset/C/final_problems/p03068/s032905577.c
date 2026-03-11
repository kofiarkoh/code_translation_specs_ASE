#include <stdio.h>
int main(void){
    int N,K,i;
    char S[10],x;
    scanf("%d %s %d",&N,S,&K);
    x=S[K-1];
    
    for(i=0;i<N;i++){
    if(S[i]!=x)
    S[i]='*';
    }     
    
    printf("%s",S);
    
    return(0);
    
}