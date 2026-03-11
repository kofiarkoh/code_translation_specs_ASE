#include<stdio.h>
 
int main(void){
  int R,G;
  if(scanf("%d",&R) == 1);
  if(scanf("%d",&G) == 1);
  if(R >=0 && G >=0 && R <= 4500 && G <= 4500){
    printf("%d",((2*G)-R));
  }
  return 0;
}