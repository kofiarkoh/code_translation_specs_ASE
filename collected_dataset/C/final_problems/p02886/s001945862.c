#include <stdio.h>
int main(void){
    int N,i,j,sum=0;
    int d[100];
    
    scanf("%d",&N);
    for(i=0;i<N;i++){
        scanf("%d",&d[i]);
    }
    
    for(i = 0;i<N;i++){
        for(j=i+1;j<=N;j++){
            sum+=d[i]*d[j];
        }
    }
    printf("%d",sum);
    return 0; 
}