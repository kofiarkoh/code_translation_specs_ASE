#include <stdio.h>
#include <stdlib.h>
#define MAX 100001

int from[MAX],fnext[MAX],to[MAX],tnext[MAX],
    flag[MAX],
    f_s[MAX][2],
    store[MAX][3],
    answer=0;

int max(int a, int b){
  return a<b ? b : a;
}

void search_A(int a){
  int p;
  flag[a] = 1;
  p = to[a];
  while(p != -1){
    if(!flag[store[p][0]])search_A(store[p][0]);
    p = tnext[p];
  }
  return;
}

int search_B(int a, int sum){
  int p,res;
  if(f_s[a][0]){
    if(sum > f_s[a][1]){
      answer = -1;
      return -1;
    }else{
      return 0;
    }
  }
  f_s[a][0] = 1;
  f_s[a][1] = sum;
  p = from[a];
  while(p != -1){
    if(flag[store[p][1]]){
      res = search_B(store[p][1],sum + store[p][2]);
      if(res == -1)return -1;
    }
    p = fnext[p];
  }
  f_s[a][0] = 0;
  return 0;
}

int main(){
  int n,m,ofs,p,
      i;
  
  scanf("%d%d%d",&n,&m,&ofs);
  
  for(i=1;i<=n;i++)from[i] = -1;
  for(i=1;i<=n;i++)to[i] = -1;
  for(i=0;i<m;i++){
    scanf("%d%d%d",&store[i][0],&store[i][1],&store[i][2]);
    store[i][2] -= ofs;
    
    p = from[store[i][0]];
    while(p != -1){
      if(store[p][1] == store[i][1])break;
      p = fnext[p];
    }
    if(p != -1){
      if(store[p][2] < store[i][2])store[p][2] = store[i][2];
    }else{
      fnext[i] = from[store[i][0]];
      from[store[i][0]] = i;
      tnext[i] = to[store[i][1]];
      to[store[i][0]] = i;
    }
  }
  
  for(i=1;i<=n;i++)flag[i] = 0;
  search_A(n);
  
  for(i=1;i<=n;i++)f_s[i][0] = 0;
  search_B(1,0);
  if(answer == 0){
    answer = max(0,f_s[n][1]);
  }
  
  printf("%d", answer);
  
  return 0;
}