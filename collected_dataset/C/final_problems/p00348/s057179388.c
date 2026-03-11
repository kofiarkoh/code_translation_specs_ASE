#include <stdio.h>
#include <stdlib.h>

int sort(int *a, int n);

int main(void){
  int i, n, ans, *a;

  scanf("%d", &n);
  a = (int *)malloc(sizeof(int) * n);
  if(a == NULL){
    printf("?????¢????¢??????¨??????\n");
    return -1;
  }
  for(i = 0; i < n; i++){
    scanf("%d", &a[i]);
  }

  ans = sort(a, n);
  printf("%d\n", ans);
  free(a);
  return 0;
}


int sort(int *a, int n){
  int i, j, m, k, k2, count = 0;
  int *set, *back;

  back = (int *)malloc(sizeof(int) * (n-1));
  if(back == NULL){
    printf("?????¢????¢??????¨??????\n");
    exit(1);
  }
  set = (int *)malloc(sizeof(int) * n);
  if(set == NULL){
    printf("?????¢????¢??????¨??????\n");
    exit(1);
  }
  for(i = 0; i < n; i++){
    set[i] = 0;
  }
  for(i = 0; i < n; i++){
    k = -1;
    for(j = 0; j < n; j++){
      if(k == -1){
	if(set[j] == 0){
	  k = j;
	}
	continue;
      }
      if(set[j] == 0 && a[k] > a[j]){
	k = j;
      }
    }
    set[k] = i+1;
  }

  for(i = 0; i < n; i++){
    k = i;
    while(set[k] != i+1){
      k++;
    }
    if(k == i){
      continue;
    }
    count += k-i;
    k2 = i;
    for(j = i; j < k; j++){
      if(set[k2] > set[j]){
	k2 = j;
      }
    }
    m = 0;
    for(j = i; j < k; j++){
      if(j == k2){
	continue;
      }
      back[m++] = set[j];
    }
    back[m] = set[k2];
    set[i] = set[k];
    m = 0;
    for(j = i+1; j < n; j++){
      if(j + (k-i) < n){
	set[j] = set[j+(k-i)];
      }else{
	set[j] = back[m++];
      }
    }
  }
  free(back);
  free(set);
  return count;
}