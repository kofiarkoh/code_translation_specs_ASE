#include <stdio.h>
#define MAX 45

int array[MAX];

void makefibo();

int main(){
  int n;

  scanf("%d", &n);

  makefibo();

  printf("%d\n", array[n]);

  return 0;
}

void makefibo(){
  int i;

  array[0] = array[1] = 1;

  for( i=2; i<MAX; i++){
    array[i] = array[i-2] + array[i-1];
  }

}