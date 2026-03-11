#include <stdio.h>
#include <stdlib.h>

int main()
{
 long i,count=0,n,temp;

 while(~scanf("%ld",&n)){
    long a[n];
    for(;n--;){
      scanf("%ld",&a[count]);
      count++;
      for (i=0;i<count/2;i++){
        temp=a[i];
        a[i]=a[count-i-1];
        a[count-i-1]=temp;
     }
    }
    for(i=0;i<count-1;i++)
      printf("%ld ",a[i]);
    printf("%ld\n",a[count-1]);
    count=0;
 }
}
