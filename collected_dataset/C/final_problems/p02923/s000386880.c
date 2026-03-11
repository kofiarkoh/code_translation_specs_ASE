#include<stdio.h>

#define N_MAX 100000

int main(void){

    int N,i,ans=0,AbleToDown = 0;
    long int H[N_MAX];

    scanf("%d",&N);
    for(i = 0;i < N;i++){
        scanf("%ld",&H[i]);
    }

    for(i = 0;i< N - 1;i++){
        if(H[i] >= H[i + 1]){
            AbleToDown++;
        }else{
            if(ans < AbleToDown){
                ans = AbleToDown;
            }
          　AbleToDown = 0;
        }
    }

    if(ans < AbleToDown){
        ans = AbleToDown;
    }

    printf("%d",ans);
    return 0;
}

