#include<stdio.h>

int main()
{
   int n,a,b;
   scanf("%d %d %d",&n,&a,&b);
   int min=(a * ( n - 1 ) + b );
   int max = a + ( n - 1 ) * b;
   if(n == 1 ){
       if( a == b)
           printf("1");
        else
            printf("0");
   } 
   else{
       if(a > b ) 
         printf("0");
       else
         printf("%lld",(long long )max - min + 1);
   
   }
    return 0;
}
    
