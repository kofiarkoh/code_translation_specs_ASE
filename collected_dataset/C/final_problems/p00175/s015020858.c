#include<stdio.h>

int main()
{
    int num,i;
    int ans[11];

    while(scanf("%d",&num),num+1){
        
        if(num==0){
            puts("0");
            continue;
        }

        for(i=0;num;i++){
            ans[i]=num%4;
            num/=4;
        }

        for(i--;i>=0;i--){
            printf("%d",ans[i]);
        }
        puts("");
    }
    return 0;
}