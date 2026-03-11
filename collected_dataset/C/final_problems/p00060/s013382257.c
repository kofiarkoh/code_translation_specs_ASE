#include <stdio.h>
int main(void){
    int c1,c2,c3,i,cnt;
    
    while(scanf("%d %d %d",&c1,&c2,&c3)!=EOF){
        for(i=1,cnt=0;i<=10;i++){
            if(i==c1||i==c2||i==c3){
                continue;
            }
            if(c1+c2+i<=20){
                cnt++;
            }
        }
        if(cnt<4)puts("NO");
        else puts("YES");
    }
    return 0;
}