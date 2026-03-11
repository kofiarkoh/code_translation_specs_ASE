#include <stdio.h>

int main(){
  long long int time[10001] = {}, wtime[10001] = {}, min, sum;
  int j, k, n;
  while(scanf("%d", &n) && n){
    for(j = 0;j < n;j++)
      scanf("%lld", &time[j]);

    for(j = 0;j < n - 1;j++){
      min = j;
      for(k = j + 1;k < n;k++)
	min = time[min] < time[k] ? min : k;
      if(min != j){
	time[min] += time[j];
	time[j] = time[min] - time[j];
	time[min] -= time[j];
      }
    }
    sum = 0;
    for(j = 1;j < n;j++){
      wtime[j] = wtime[j - 1] + time[j - 1];
      sum += wtime[j];
    }
    
    printf("%lld\n", sum);
  }
  return 0;
}