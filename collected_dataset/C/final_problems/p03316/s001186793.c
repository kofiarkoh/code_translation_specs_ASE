#include <stdio.h>

int power(int,int); /*べき乗*/

int main(void){
    
    int num;
    int i;
    int list[256];
    int ans=0;
    
    scanf("%d",&num);
    
    for(i=0;i<9;i++){
        list[i]=num/power(10,i);
        ans=ans+list[i]%10;
    }
    
    if(num%ans==0){
        printf("Yes\n");
    }else{
        printf("No\n");
    }
    
    return 0;
}


int power(int a,int b){
    
    int c,i;
    
    c = a;
    
    if(b==0){
        a=1;
    }else{
        for(i=1;i<b;i++){
            a=a*c;
        }
    }
    
    return a;
}