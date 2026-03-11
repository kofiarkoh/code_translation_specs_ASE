#include<stdio.h>
int main(){
  int X,Y;
  scanf("%d",&X);
  scanf("%d",&Y);
  
  if(Y-2*X >= 0 && (Y-2*X) % 2 == 0){
   if(4*X-Y >= 0 && (4*X-Y) % 2 == 0){
  	printf("Yes\n");
   }else{
    printf("No\n"); 
   }
  }else{
   printf("No\n"); 
  }
 return 0; 
}