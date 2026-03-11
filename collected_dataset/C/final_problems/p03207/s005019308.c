#include<stdio.h>

int main(){
    int N;
    scanf("%d",&N);
    int p[N];
    for(int i = 0;i < N;i++){
        scanf("%d",&p[i]);
    }
    
    int max = 0;
    int memo;
    for(int i = 0;i < N;i++){
        if(max < p[i]){
            max = p[i];
            memo = i;
        }
    }

    p[memo] /= 2;

    int Sum = 0;
    for(int i = 0;i < N;i++){
        Sum += p[i];
    }

    printf("%d\n",Sum);
    return 0;
}
