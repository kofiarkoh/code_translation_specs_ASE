#include <stdio.h>


int main (void){
    int n,h,w;
    scanf("%d%d%d",&n,&h,&w);
    int count = 0;
    for(int i = 0; i < n;i++){
        int a,b;
        scanf("%d%d",&a,&b);
        if(a >= h && b >= w){
            count++;
        }
    }

    printf("%d",count);

    return 0;
}
