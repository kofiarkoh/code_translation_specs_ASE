#include<stdio.h>
int main()
{
  int A,B;
  scanf("%d\n%d",&A,&B);
  
  if(A==1){
    if(B==2){printf("3");}
    if(B==3){printf("2");}
  }else if(A==2){
    if(B==1){printf("3");}
    if(B==3){printf("1");}
  }else if(A==3){
    if(B==1){printf("2");}
    if(B==2){printf("1");}
  }
}
