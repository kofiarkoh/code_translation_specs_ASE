#include <stdio.h>

int main(void){

        int h,w;
        int n;
        int ans;

        scanf("%d%d%d",&h,&w,&n);

        int longer = h;

        if(longer<w){
                longer = w;
        }   

        ans = n/longer;

        if(n%longer != 0){ 
                ans++;
        }   

        printf("%d\n",ans);

return 0;
}
