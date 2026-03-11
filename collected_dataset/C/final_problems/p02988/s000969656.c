#include<stdio.h>
int main(void){
    int n,count = 0;
    scanf("%d", &n);
    int p[n];
    for(int i = 0 ; i < n ; i++){
        scanf("%d", &p[i]);
    }
    for(int i = 1 ; i < n-1 ; i++){
        if(p[i-1] > p[i] && p[i] > p[i+1]){
            count++;
        }else if(p[i-1] < p[i] && p[i] < p[i+1]){
            count++;
        }
    }
    printf("%d\n",count);
    return 0;
}