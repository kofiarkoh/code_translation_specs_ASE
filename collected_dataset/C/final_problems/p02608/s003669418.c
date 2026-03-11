#include <stdio.h>
int func(int n);
int main(void){

    int N;
    scanf("%d",&N);

    for(int i=1;i<=N;i++){ 
        printf("%d\n",func(i));
    }

    return 0;
}
int func(int n){
    int count=0;
    for(int i=1;i*i<=n;i++){
        for(int j=1;j*j<=n;j++){
            for(int k=1; k*k<=n;k++){
                //printf("%d %d %d \n",i,j,k);
                if(i*i+j*j+k*k+i*j+j*k+k*i==n){count++;}
            }
        }
    }
    return count;
}