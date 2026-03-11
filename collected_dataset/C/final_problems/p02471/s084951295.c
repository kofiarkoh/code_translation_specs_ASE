#include <stdio.h>

int main(void){
    int a,b,c,q;
    int x1,x2,y1,y2,z;
    int flg=0;
    scanf("%d %d",&a,&b);
    x1=1;x2=0;y1=0;y2=1;
    if(a<b){c=a;a=b;b=c;flg=1;}
    while(1){
        q=a/b;c=a%b;
        if(c==0){break;}
        z=x1-x2*q;x1=x2;x2=z;
        z=y1-y2*q;y1=y2;y2=z;
        a=b;b=c;
    }
    if(flg==1){c=x2;x2=y2;y2=c;}
    printf("%d %d\n",x2,y2);
}
