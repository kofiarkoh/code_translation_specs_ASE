#include <stdio.h>

int Check(int array[], int size){
  int i, cnt = 0;
  for(i = 0; i < size; i++){
    if(array[i] != 0) cnt++;
    if(cnt >= 2) return 2;
  }
  if(cnt == 1) return 1;
  else return 0;
}

void solve(int n, int p){
  int i;
  int hand[n], bowl;
  bowl = p;
  
  for(i = 0; i < n; i++){
    hand[i] = 0;
  }
  i = 0;
  while(1){
    if(bowl == 1){
      bowl--;
      hand[i]++;
      if(Check(hand, n) == 1){
	printf("%d\n", i);
	return;
      }
    }
    else if(bowl == 0){
      bowl = hand[i];
      hand[i] = 0;
    }
    else{
      bowl--;
      hand[i]++;
    }

    i = (i + 1) % n;
  }
}

int main(void){
  int n, p;
  while(1){
    scanf("%d %d", &n, &p);
    if( n == 0 && p == 0)
      return 0;
    else 
      solve(n, p);
  }
}
