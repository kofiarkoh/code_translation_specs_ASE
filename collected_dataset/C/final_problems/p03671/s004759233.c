#include <stdio.h>

int main(){
    int a,b,c;
    int p1,p2,p3,min;


    scanf("%d%d%d",&a,&b,&c);
    
    p1 = a+b;
    p2 = b+c;
    p3 = a+c;

    if(p1<p2){
        if(p1<p3){
            min = p1;
        }
        else if(p3<p1){
            min = p3;
        }
    }
    else if(p2<p1){
        if(p2<p3){
            min = p2;
        }
    }

    printf("%d\n",min);

    return 0;
}