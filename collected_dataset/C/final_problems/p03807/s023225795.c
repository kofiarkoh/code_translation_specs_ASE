#include<stdio.h>
int main(void){
    int n,a,ki;
    scanf("%d",&n);
    for(a=0;n>0;n--){
        scanf("%d",&a);
        if(a%2==1){ki++;}
    }
    if(ki%2==0){printf("YES\n");}else{printf("NO\n");}
    return 0;
}