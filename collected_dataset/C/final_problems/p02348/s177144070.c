#include<stdio.h>
#include<stdlib.h>
#include<math.h>


int main(void){

  int a[100000]={};
  int min[100000]={};
  int i=0;  
  int time=0;
  int b,x,s,t,n,q,d,c;

  scanf("%d %d",&n,&q);
  if(1>n||n>100000||1>q||q>100000){
  printf("Too Large\n");
  exit(0);
  }

  while(i<n){
    a[i]=pow(2,31)-1;
    min[i]=a[i];
    i++;
  }

  i=0;




  while(i<q){
    i++;
   scanf("%d",&b);

   if(b==0){
   scanf("%d",&d);
   scanf("%d",&c);
   scanf("%d",&x);
   if(d>c||c>=n||x>=pow(2,23)-1){
   printf("Too Large\n");
   exit(0);
   }
   while(d<=c){
   a[d] = x;
   d++;
   }

   }
 
   else if(b==1){
     time++;
   scanf("%d",&s);
   if(s>=n){
   printf("Too Large\n");
   exit(0);
   }
   min[time]=a[s];
   }

   }


  

  i=0;

  while(i<time){
    ++i;
    printf("%d\n",min[i]);

  }
  


}