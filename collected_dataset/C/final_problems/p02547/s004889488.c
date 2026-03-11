#include <stdio.h>
#include <math.h>
#include <stdlib.h>
int main()
{   long long n;
    scanf("%lld",&n);
    int cnt=0;
    for(int i=0;i<n;i++){
        int tmp1,tmp2;
        scanf("%d%d",&tmp1,&tmp2);
        if(tmp1==tmp2){
            cnt++;
        }else{
            cnt=0;
        }
        if(cnt==3){
            printf("Yes");
            return 0;
        }
    }
    
    printf("No");
    return 0;
}
