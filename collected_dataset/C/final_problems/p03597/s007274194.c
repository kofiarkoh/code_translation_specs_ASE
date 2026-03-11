/* ex2_3
   moka223711 */

#include <stdio.h>

int main(void){
  // N * N はマス目の数を表す
  // マス目のAマスを白く塗り、残りを黒く塗る
  int N, A;
  scanf("%d%d", &N, &A);
  
  // N * N - A は黒く塗るマスの数
  printf("%d\n", N * N - A);
  
  return 0;
}
