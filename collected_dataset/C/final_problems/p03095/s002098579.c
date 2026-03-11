#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long pow2(int l);
void input(int *n, char s[]);
 
int main(int argc, char *argv[]){
  int n, amounts[100000], l, tf;
  char s[100001], lit[100000];

  long sum, subsum;

  long i, j, k;

  input(&n, s);

  lit[0] = s[0];
  amounts[0] = 1;
  l = 1;

  for(i = 1; i < n; i++){
    tf = 1;
    for(j = 0; j < l; j++){
      if(s[i] == lit[j]){
        amounts[j] ++;
        tf = 0;
        break;
      }      
    }
    if(tf){
      lit[l] = s[i];
      amounts[l] = 1;
      l++;
    }
  }

  sum = 0;

  for(i = 1; i <= pow2(l); i++){
    subsum = 1;
    j = 0;
    for(k = i; k > 0; k >>= 1){
      if(k % 2 == 1){
        subsum = ((subsum % 1000000009) * (amounts[j] % 1000000009) % 1000000009);
      }
      j ++;
    }
    sum += subsum;
    subsum %= 1000000009;
  }

  printf("%ld\n", sum);
 
  return 0;
}
 
/////////////////////////////////////////////////////////////////////////

long pow2(int l){
  long x = 1;
  for(int i = 0; i < l; i++){
    x <<= 1;
  }
  return x;
}

/////////////////////////////////////////////////////////////////////////

void input(int *n, char s[]){
  scanf("%d", n);
  scanf("%s", s);
}