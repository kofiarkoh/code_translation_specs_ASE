#include <stdio.h>
int main(){
    unsigned int a,b,k;
    scanf("%d %d %d", &a, &b, &k);
    if(k<=a){
        printf("%d %d", a-k,b);
    }else if(a<k && k <= a+b){
        printf("0 %d", a+b-k);
    }else if(a+b < k){
        printf("0 0 ");
    }
}