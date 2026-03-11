#include<stdio.h>

int main(){
    int x,y,result,i;
    scanf("%d %d",&x,&y);
    result=x+y+2;
    for(i=0;i<=x;i++){
        if((y*i)%x==0){
            result-=1;
        }
    }
    printf("%d\n",result);
    return 0;
}
