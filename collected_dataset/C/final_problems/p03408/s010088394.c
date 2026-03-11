#include <stdio.h>

int main(void){
  int N, M;
  char s[100][10] = {};//青いカード
  char t[100][10] = {};//赤いカード
  int i, j, k;
  int max = -1000;
  int count;
  int flag;
  scanf("%d", &N);
  for(i = 0; i < N; i++){
    scanf("%s", &s[i][0]);
  }
  scanf("%d", &M);
  for(i = 0; i < M; i++){
    scanf("%s", &t[i][0]);
  }

  for(i = 0; i < N; i++){
    count = 0;
    for(j = 0; j < N; j++){
      //s[i][]とs[j][]が一致してるか確認
      flag = 1;
      for(k = 0; k < 10; k++){
        if(s[i][k] != s[j][k]){
          flag = 0;
          break;
        }
      }
      //ここでflag == 1ならば，s[i][]とs[j][]は一致
      if(flag == 1){
        count++;
      }
    }
    for(j = 0; j < M; j++){
      //s[i][]とt[j][]が一致してるか確認
      flag = 1;
      for(k = 0; k < 10; k++){
        if(s[i][k] != t[j][k]){
          flag = 0;
          break;
        }
      }
      //ここでflag == 1ならば，s[i][]とt[j][]は一致
      if(flag == 1){
        count--;
      }
    }
    if(count > max){
      max = count;
    }
  }

  for(i = 0; i < M; i++){
    count = 0;
    for(j = 0; j < N; j++){
      //t[i][]とs[j][]が一致してるか確認
      flag = 1;
      for(k = 0; k < 10; k++){
        if(t[i][k] != s[j][k]){
          flag = 0;
          break;
        }
      }
      //ここでflag == 1ならば，t[i][]とs[j][]は一致
      if(flag == 1){
        count++;
      }
      //t[i][]とt[j][]が一致してるか確認
      flag = 1;
      for(k = 0; k < 10; k++){
        if(t[i][k] != t[j][k]){
          flag = 0;
          break;
        }
      }
      //ここでflag == 1ならば，t[i][]とt[j][]は一致
      if(flag == 1){
        count--;
      }
    }
    if(count > max){
      max = count;
    }
  }

  if(max < 0){
    max = 0;
  }
  
  printf("%d\n", max);
  return 0;
}
