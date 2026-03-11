#include<stdio.h>
int main(){
    int a,b,x;

    scanf("%d %d %d",&a,&b,&x);

    if(a<b){
        if(x%1000)printf("%d\n",a*(x/1000+1));
        else printf("%d\n",a*(x/1000));
    }
    else if(a>=b&&a<2*b){
        int tmp=0;
        tmp=a*(x/1000);
        x%=1000;
        //printf(">>%d\n",x);
        if(x>500)printf("%d\n",tmp+a);
        else if(0<x) printf("%d\n",tmp+b);
        else printf("%d\n",tmp);
    }else if(a>=2*b){
        if(x%500)printf("%d\n",b*(x/500+1));
        else printf("%d\n",b*(x/500));
    }
    return 0;
}
