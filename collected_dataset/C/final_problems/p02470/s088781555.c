#include <stdio.h>
#include <math.h>

int main(void){
  int i = 2, j = 0, k, d, n, ncpy, s[10], count = 1;

  scanf("%d", &n);
  ncpy = n;
  while(ncpy != 1){
    if(ncpy % i == 0){
      while(ncpy % i == 0){
	ncpy /= i;
      }
      s[j++] = i;
      i++;
    }else if(i > sqrt(ncpy)){
      s[j++] = n;
      break;
    }else{
      i++;
    }
  }

  for(i = 2; i < n; i++){
    d = 0;
    for(k = 0; k < j; k++){
      if(i % s[k] == 0){
	d++;
       	break;
      }
    }
    if(d == 0){
      count++;
    }
  }
  printf("%d\n", count);
  return 0;
}