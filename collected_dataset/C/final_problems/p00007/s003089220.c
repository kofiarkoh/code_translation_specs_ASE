#include<stdio.h>
  int main(void){
    int a=100000,b,m,n,i,j;
     scanf("%d",&n);
     for(i=0;i<n;i++){
      a=a+a*0.05;
      if(a%1000!=0)
      a=(a/1000+1)*1000;


        }

printf("%d\n",a);

 return 0;
 }