#include<stdio.h>
int main(){
    int n;
    long int sum = 0;
    scanf("%d",&n);
    for(int i = 1;i <= n;i++){
        if(i % 3 == 0 || i % 5 == 0){
            continue;
        }
        //printf("i = %d %d\n",i,sum);
        sum += i;
    }
    printf("%lld\n",sum);

}