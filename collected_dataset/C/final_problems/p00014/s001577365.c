#include<stdio.h>
int main(void){
    int d,i,s=0;
    while(scanf("%d",&d)!=EOF){
        i=0;
        s=0;
        while(i<600){
            s+=d*i*i;
            i+=d;
        }
        /*if(d==600){
            s=d*600*600;
        }
        */
        printf("%d\n",s);
    }
return 0;
}