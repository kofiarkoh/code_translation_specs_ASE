#include<stdio.h>

int main(void){
    int i,N,counter = 0;
    scanf("%d",&N);
    int a[N];
    for(i=0;i<N;i++){
        scanf("%d",&a[i]);
        counter = counter + a[i] + -1;
    }
    printf("%d\n",counter);
    return 0;

}