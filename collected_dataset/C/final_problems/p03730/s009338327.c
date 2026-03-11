#include<stdio.h>
int main(void){
    int a,b,rem,fg,i,tem;
    fg=0;
    scanf("%d %d %d",&a,&b,&rem);
    for(i=1;i<=b;i++){
        tem=a*i;
        if(rem==tem%b){
            fg=1;
            break;
        }
    }
    if(fg==1){
        printf("YES\n");
    }else{
        printf("NO\n");
    }
    return 0;
}

