 /*ex2_3*/
/*moriai ryo*/
#include <stdio.h>
 
int main(void){
  int size,white;/*変数用意*/
  scanf("%d",&size);
  scanf("%d",&white);/*入力*/
  printf("%d\n",size*size-white);/*一辺^2-白いマスの数*/
  return 0;
}
