#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

int main(void){
    int i,j,k;
    int N,a[200001],count = 0;
    bool check[200001] = {false};
    
    scanf("%d",&N);
    for(i = 0;i < N - 1;i++){
        scanf("%d\n",&a[i]);
    }
    scanf("%d",&a[i]);
    
    for(i = N-1;i >= 0;i--){
        count = 0;
        for(j = i;j < N;j += (i+1)){
            if(check[j] == true){
                count++;
            }
        }
        if(a[i] == 1){
            if(count % 2 == 0){
                check[i] = true;
            }
        }else{
            if(count % 2 == 1){
                check[i] = true;
            }
        }
    }
    
    count = 0;
    for(i = 0;i < N;i++){
        if(check[i] == true){
            count++;
        }
    }
    printf("%d\n",count);
    for(i = 0;i < N;i++){
        if(check[i] == true){
            printf("%d ",i+1);
        }
    }
    printf("\n");
    
    return 0;
}

