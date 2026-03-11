#include<stdio.h>

int main(void){
int N;
scanf("%d\n", &N);
char S,tmp;
scanf("%1c", &S);
if(N == 1){
  printf("0");
  return 0;
}

int cnt = 0;
for(int i = 0; i < N - 1; i++){
  scanf("%1c", &tmp);
  //printf("S = %c tmp = %c\n", S, tmp);
  if(S == '#' && tmp == '.'){
    cnt++;
    S = '#';
  }
  else{
    S = tmp;
  }
}
printf("%d",cnt);
}