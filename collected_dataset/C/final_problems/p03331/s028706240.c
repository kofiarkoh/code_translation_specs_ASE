#include <stdio.h>

int main(){
    int e;
    int x;
    int er;
    int is;
    int es;
    int data;
    is=0;
    es=0;
    data=100000;
    scanf("%d",&e);
        for(int i=2;i<=e-2;i++){
            is=0;
            es=0;
            x=i;
            er=e-x;
            while(x>0){
                is=is+x%10;
                x=x/10;
            }
            while(er>0){
                es=es+er%10;
                er=er/10;
            }
            if(is+es<data){
                data=is+es;
            }
        }
    printf("%d\n",data);
    return 0;
}
