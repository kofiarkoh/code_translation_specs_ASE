#include <stdio.h>

int main(void){
    int n,x,y,h,w,sum,sum1,i;
    for(;sum=0,scanf("%d",&n),n;){
        for(i=0;i<n;i++){
            scanf("%d %d %d %d",&x,&y,&h,&w);
            sum1=x+y+h;
            if(sum1 <= 60 && w <= 2)
                sum+=600;
            else if(sum1 <= 80 && w <= 5)
                sum+=800;
            else if(sum1 <= 100 && w<= 10)
                sum+=1000;
            else if(sum1 <= 120 && w<= 15)
                sum+=1200;
            else if(sum1 <= 140 && w<=20)
                sum+=1400;
            else if(sum1 <= 160 && w<=25)
                sum+=1600;
        }
        printf("%d\n",sum);
    }
    return 0;
}