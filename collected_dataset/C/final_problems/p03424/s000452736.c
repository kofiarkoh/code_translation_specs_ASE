
#include <stdio.h>

int main()
{
   long int a,b[4]={0},c,x,y=0,flag=0,i,j,count,sum,max;
  char k[100];
   scanf("%ld\n",&a);
   gets(k);
   for(i=0;i<a+(a-1);i++)
   {
       
       if(k[i]=='P')
       {
           b[0]=1;
       }
       if(k[i]=='W')
       {
         b[1]=1;
       }
       if(k[i]=='G')
       {
           b[2]=1;
       }
       if(k[i]=='Y')
       {
           b[3]=1;
       }
      
   }
   
   if(b[0]+b[1]+b[2]+b[3]==4)
   {
       printf("Four");
   }else{
       printf("Three");
   }
   
     return 0;
}
