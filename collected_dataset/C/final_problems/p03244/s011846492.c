#include<stdio.h>
#include<stdlib.h>

int compare_int(const void *a, const void *b){
  return *(int*)a - *(int*)b;
}

int main(){
  int N;
  scanf("%d", &N);

  int O[N/2 + 1], E[N/2 + 1];
  O[N/2] = 0;
  E[N/2] = 0;

  int i, j;

  for(i = 0; i < N; i++){
    if(i%2 == 1) scanf("%d", &O[i/2]);
    else scanf("%d", &E[i/2]);
  }

  qsort(O, N/2, sizeof(int), compare_int);
  qsort(E, N/2, sizeof(int), compare_int);
  
  int max[2][2][2] = {};

  int count_O = 1, count_E = 1;

  for(i = 0; i < N/2; i++){
    if(O[i] != O[i+1]){
      if(max[0][0][0] < count_O){
        max[0][1][0] = max[0][0][0];
        max[0][1][1] = max[0][0][1];
        max[0][0][0] = count_O;
        max[0][0][1] = O[i];
      } else if(max[0][1][0] < count_O){
        max[0][1][0] = count_O;
        max[0][1][1] = O[i];
      }
      count_O = 1;
    } else count_O++;

    if(E[i] != E[i+1]){
      if(max[1][0][0] < count_E){
        max[1][1][0] = max[1][0][0];
        max[1][1][1] = max[1][0][1];
        max[1][0][0] = count_E;
        max[1][0][1] = E[i];
      } else if(max[1][1][0] < count_E){
        max[1][1][0] = count_E;
        max[1][1][1] = E[i];
      }
      count_E = 1;
    } else count_E++;
  }
  int ans;
  if(max[0][0][1] == max[1][0][1]){
    if(max[0][0][0] < max[1][0][0]){
      ans = N - max[1][0][0] - max[0][1][0];
    } else ans = N - max[0][0][0] - max[1][1][0];
  } else ans = N - max[0][0][0] - max[1][0][0];

  printf("%d\n", ans);

  return 0;
}
