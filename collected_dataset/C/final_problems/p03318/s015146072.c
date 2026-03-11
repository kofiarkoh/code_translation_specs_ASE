#include<stdio.h>
int main(void){
    int a,b,c[100000],d;
    scanf("%d",&a);
    if(a<10){
    for(int i=1;i<a;i++){
        printf("%d",i);
    }
    }
    if(10<=a){
        for(int i=1;i<10;i++){
        printf("%d\n",i);
    }
        for(int j=1;j<a-8;j++){
            printf("%d\n",9+10*j);
        }
    }
    return 0;
}
