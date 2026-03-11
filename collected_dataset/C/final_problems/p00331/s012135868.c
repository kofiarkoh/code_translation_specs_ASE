#include<stdio.h>
int main(void)
{
    int H,R,a,b;
    scanf("%d %d",&H,&R);
    a=H+R;
    if(a>0){
        b=1;
    }
    else if(a==0){
        b=0;
    }
    else{
        b=-1;
    }
    printf("%d",b);
    return(0);
}