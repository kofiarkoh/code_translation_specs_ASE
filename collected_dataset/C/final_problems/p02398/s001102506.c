#include <stdio.h>
 
int main(void) {
    int a,b,c;
 
    int i,num= 0;
    scanf("%d",&a);
    scanf(" %d",&b);
    scanf(" %d",&c);
     
    for(i=a;i <= b;i++){
         
     
        if(c % i == 0){
            num++;
        }
     
    }
     
    printf("%d\n",num);
    return 0;
}