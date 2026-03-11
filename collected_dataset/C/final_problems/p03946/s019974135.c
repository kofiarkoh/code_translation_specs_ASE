#include <stdio.h>
#include <stdlib.h>

int main(){
    int N,T;
    scanf("%d %d",&N,&T);
    if(T < 2){
        printf("0\n");
        return 0;
    }
    int * cost = malloc(N*sizeof(int));
    int i;
    for(i = 0; i < N; i++){
        scanf("%d",&cost[i]);
    }
    
    //find max profit
    int j;
    int max = 0;
    for(i = 0; i < N-1; i++){
        for(j = i+1; j<N; j++){
            if( (cost[j]-cost[i])>max) max = (cost[j]-cost[i]);
        }
    }
    
    //and now count how many times that appears
    int count = 0;
    for(i = 0; i < N-1; i++){
        for(j = i+1; j<N; j++){
            if( (cost[j]-cost[i])==max) count++;
        }
    }
    
    printf("%d\n",count);
    return EXIT_SUCCESS;
}