#include <stdio.h>
int main(void){
   int n,a,i;
    int  min = 1000000;
    int  max = 0;
    long int sum = 0;
  
  scanf("%d",&i);
   
   for (n=1; n<=i; n++){
   
   scanf("%d",&a);
   
   if(a < min) {
    min = a;
    }
    
  
   if(a > max) {
    max = a;
    }
    
    sum = sum + a;
   }
    
    printf("%d %d %lld\n",min, max, sum);
   
    return 0;
    
     
}



