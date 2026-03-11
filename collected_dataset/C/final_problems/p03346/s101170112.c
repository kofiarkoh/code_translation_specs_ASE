#include <stdio.h>

#define MAX 20000

int main(){
  int array[MAX+1],temp[MAX+1];
  int n;
  int i,j,count = 0;

  scanf("%d", &n);
  for(i = 1; i <= n+1; i++){
    scanf("%d", &array[i]);
  }

  for(i = 0; i <= n; i++){
    for(j = 0; j <= n; j++){
      if(i == array[j]){
        temp[i] = j;
        break;
      }
    }
  }

  // for(i = 1; i <= n; i++){
  //   printf("%d", temp[i]);
  // }

  int tp, pans = 0;
  tp = temp[1];

  for(i = 2; i <= n; i++){
    count++;
    // printf("%d%d\n", tp, temp[i]);

    if(tp > temp[i]){
      count = 0;
    }

    tp = temp[i];
    // printf("%d\n", count);
    if(count > pans) pans = count;
  }

   int ans;
   ans = n - pans - 1;

   printf("%d", ans);

  return 0;
}
