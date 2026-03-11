//4doll cake 7doll donatu
#include<stdio.h>
int main(void){
    int N,onoff=0;

    scanf("%d",&N);
    int i,j;
    for(i=0;i<N;i++){
        for(j=0;j<N;j++){
            if(4*i+j*7==N)onoff=1;
        }
    }
    if(onoff==1)printf("Yes");
    else printf("No");
    return 0;
}