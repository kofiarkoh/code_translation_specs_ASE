#include <stdio.h>

void del(int *v, int l, int del){
  int i;
  for(i = 0; i < l - 1; i++){
    if(i >= del){
      v[i] = v[i + 1];
    }
  }
  return;
}

int slv(int *v, int v_l, int *sol, int sol_l){
  int i, j, flg;
  if(v_l == 1 && v[0] == 1){
    sol[0] = 1;
    return 1;
  }
  for(i = v_l - 1; i >= 0; i--){
    flg = 0;
    if(v[i] == i + 1){
      int newV[v_l];
      for(j = 0; j < v_l; j++){
        newV[j] = v[j];
      }
      del(newV, v_l, i);
      sol[v_l - 1] = i + 1;
      flg = slv(newV, v_l - 1, sol, sol_l);
      //printf("%d %d",i,  flg);
    }
    if(flg){return 1;}
  }
  return 0;
}

int main(){
  int n, i;
  scanf("%d", &n);
  int b[n];
  for(i = 0; i < n; i++){
    scanf("%d", &b[i]);
  }
  int sol[n];
  for(i = 0; i < n; i++){
    sol[i] = -1;
  }
  if(slv(b, n, sol, n)){
    for(i = 0; i < n; i++){
      printf("%d\n", sol[i]);
    }
  }else{
    printf("-1\n");
  }
  return 0;
}
