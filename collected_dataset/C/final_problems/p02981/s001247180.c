#include <stdio.h>


int main(void){
    int n,a,b;
    int ans;

    scanf("%d%d%d",&n,&a,&b);
    
    if(n*a > b){
        ans = b;
    } else {
        ans = n*a;
    }
    printf("%d",ans);
    return 0;
}