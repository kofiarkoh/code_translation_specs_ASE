#include <stdio.h>
#include <stdlib.h>
#include <math.h>


int main(void) {
  int n, i, j;
  int pn[200000];
  char str_n[7];
  char str_pn[7];
  int min=200000;
  int ans=0;
  int dig;
  int top, middle, bottom;
  int temp;
  int dig_a, dig_b;
  int i_dig;
  int c[10][10];
  
  
  
  //printf("%d", (int)log10(9));
  scanf("%s", str_n);
  n = atoi(str_n);
  dig = (int)log10(n);


  bottom = n%10;
  top=n;
  for(i=0; i<dig; i++) {
    top = top/10;
  }  
  middle = (n - bottom - top*(int)pow(10,dig))/10;

  if(dig==0) {
    printf("%d\n", n);
  } else {
    for(i=1; i<10; i++) {
      for(j=1; j<=i; j++) {
        //printf("i: %d j: %d", i, j);
        c[i][j] = 0;
        c[j][i] = 0;
        if(i==j) {
          c[i][j] += 1;
          c[j][i] += 1;
        }
        //printf(" ; %d", c[i][j]);
        for(i_dig=1; i_dig<dig; i_dig++) {
          c[i][j] += (int)pow(10, i_dig-1);
          c[j][i] += (int)pow(10, i_dig-1);
        }
        //printf(" ; %d", c[i][j]);
        temp = top*10+bottom;
        if(top > i) {
          c[i][j] += (int)pow(10, (dig-1));
        } else if(top < i) {
          c[i][j] += 0;
        } else if(bottom > j) {
          c[i][j] += middle+1;
        } else if(bottom < j){
          c[i][j] += middle;
        } else {
          c[i][j] += middle+1;
        }
        if(top > j) {
          c[j][i] += (int)pow(10, (dig-1));
        } else if(top < j) {
          c[j][i] += 0;
        } else if(bottom > i) {
          c[j][i] += middle+1;
        } else if(bottom < i){
          c[j][i] += middle;
        } else {
          c[j][i] += middle+1;
        }
        //printf(" ; %d\n", c[i][j]);
        if(i==j) {
          c[i][j] /= 2;
        }
      }
    }
    for(i=1; i<10; i++) {
      for(j=1; j<10; j++) {
        ans += c[i][j] * c[j][i];
       }
    }  
    printf("%d\n", ans);
  }

  return 0;

}