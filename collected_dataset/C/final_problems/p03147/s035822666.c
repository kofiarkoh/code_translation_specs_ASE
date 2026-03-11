#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#define MAX(X, Y) (((X) > (Y))? (X) : (Y))
#define MIN(X, Y) (((X) < (Y))? (X) : (Y))
//　90min
int main(void){
  int n;
  int h[101] = {0};
  int a[101] = {0};
  int sum;
  int l = 0, r = 0;

  scanf("%d", &n);
  
  for(int i = 0; i < n; i++){
    scanf("%d", &h[i]);
  }

  
  while(1){
    r = l;
    sum++;
    //どこまで足すか決める
    for(int i = l; i < n; i++){
      if(h[i] - a[i] > 0){
	r++;
      }else{
	break;
      }
      //      printf("h[%d] = %d, a[%d] = %d, r = %d\n", i, h[i], i, a[i], r);
    }

    //足す
    for(int i = l; i < r; i++){
      a[i]++;
    }
    //    printf("a sum = %d \n", sum);
    //もう足さなくていいところを確認
    for(int i = 0; i < n; i++){
      //      printf("h[%d] = %d, a[%d] = %d, l = %d\n", i, h[i], i, a[i], l);
      if(h[i] == a[i]){
	l = i + 1;
	if(i == n - 1){
	  printf("%d\n", sum);
	  return 0;
	}
      }else{
	//	printf("break h[%d] = %d, a[%d] = %d, l = %d\n", i, h[i], i, a[i], l);
	break;
      }
    }
  }
  
  //  printf("%d\n", sum);

  return 0;
}