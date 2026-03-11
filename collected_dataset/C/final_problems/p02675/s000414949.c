/*  ex3_2
    yy8332  */

#include <stdio.h>

int main(void){
  int number;

  //入力受け取り
  scanf("%d", &number);

  //下一桁が2,4,5,7,9の場合
  switch(number % 10){
    case 2: case 4: case 5: case 7: case 9:
      printf("hon");
      break;
    //下一桁が0,1,6,8の場合
    case 0: case 1: case 6: case 8:
      printf("pon");
      break;
    //下一桁が3の場合
    case 3:
      printf("bon");
      break;
  }



return 0;
}
