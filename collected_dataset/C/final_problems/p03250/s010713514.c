#include <stdio.h>
int main(void){
  int A, B,C,num_1,num_2,num_3,D;
  int result;
  scanf("%d%d%d", &A,&B,&C);
  num_1 =A;
  num_2 =B;
  num_3 =C;


  if (num_1 < num_2){
    num_1 = B;
    num_2 =A;
  }

  if (num_1 < num_3){
    num_1 =C;
    num_3 =A;
  }

  if (num_2 < num_3){
  D = num_2 ;
    num_2 =num_3;
    num_3 =D;
  }

  result = num_1*10 + num_3 +num_2;

  printf("%d\n",result );

  return 0;
}