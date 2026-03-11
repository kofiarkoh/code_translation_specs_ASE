#include<stdio.h>
int main(){
    int i;
    long n,t;
    long time[200000];
    long sum = 0;
    
    scanf("%ld %ld",&n,&t);
    for(i = 0; i < n; i++) scanf("%ld",&time[i]);
    
    for(i = 0; i < n-1; i++){
        if(time[i+1] - time[i] <= t){ //お湯が流れている間にスイッチが押される
            sum += time[i+1] - time[i];
           // printf("type 1 sum = %ld\n",sum);
        }else{
            sum += t;
           // printf("Type 2 sum = %ld\n",sum);
        }
    }
    sum += t; //最後の人
    printf("%ld\n",sum);
    return 0;
}
