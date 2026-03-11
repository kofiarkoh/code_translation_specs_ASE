#include <stdio.h>

int main(void){
    int a,b,c,sign,start,end,t=0;
    scanf("%d %d %d",&a,&b,&c);
    sign=60%(a+b);
    start=0;
    end=a;
    do{
        if(start <= c && c <= end){
            printf("%d\n",c);
            return 0;
        }
        start=end+b;
        end=start+a;
    }while(start <= c);
    end=end-a-b;
    if(sign == 0)
        printf("-1\n");
    else{
        while(end < c){
            end+=sign;
            t++;
        }
        printf("%d\n",c+60*t);
    }
    return 0;
}