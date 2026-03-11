#include <stdio.h>
int main (void){
        double X,Y;
        int test,ans=0;
        scanf("%lf",&X);
        scanf("%lf",&Y);
        test=Y/X;
        while (1) {
                if(test==0) {
                        break;
                }else{
                        test/=2;
                        ans++;
                }
        }

        printf("%d",ans);
        return 0;
}
