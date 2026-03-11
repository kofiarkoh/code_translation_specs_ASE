#include<stdio.h>
int main(){
    int age0,age1,age2,age3,age4,age5,age6,n,i,x;
    while(1){
        age0=0;age1=0;age2=0;age3=0;age4=0;age5=0;age6=0;
        scanf("%d",&n);
        if(n==0)break;
        for(i=0;i<n;i++){
            scanf("%d",&x);
            if(x>=60){
                age6++;
            }else if(x>=50){
                age5++;
            }else if(x>=40){
                age4++;
            }else if(x>=30){
                age3++;
            }else if(x>=20){
                age2++;
            }else if(x>=10){
                age1++;
            }else{
                age0++;
            }
        }
        printf("%d\n%d\n%d\n%d\n%d\n%d\n%d\n",age0,age1,age2,age3,age4,age5,age6);
    }
    return 0;
}