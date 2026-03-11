#include <stdio.h>
#include <math.h>
 
int main(){
 
  /* 三角形の辺 */
  int a, b, c;
 
  /* 辺の長さを入力 */
  scanf("%d %d %d",&a,&b,&c);
 
  /* 三角形の種類を判別・出力 */
  if ((a + b > c && a + c > b && b + c > a)&& (a > 0 && b > 0 && c > 0)&&(a == b && b == c)) 
  {
  	printf("Yes");
  }
  
  else 
  {
    printf("No");
  }
   return 0;
}
