#include<stdio.h>

long long abs(long long a){
  if(a >= 0){
    return a;
  }
  else{
    return -a;
  }
}

int main(void){
  int n , k;
  scanf("%d %d",&n ,&k);

  long long x[n];
  for(int i = 0;i < n;i ++){
    scanf("%lld",&x[i]);
  }

  int start , end;
  long long time , min = 1000000000000;
  
  for(int i = 0;i < n - k + 1;i ++){
    start = i;
    end = i + k - 1;
    if(abs(x[start]) < abs(x[end])){
      time = abs(x[start]) + abs(x[start] - x[end]);
    }
    else{
      time = abs(x[end]) + abs(x[start] - x[end]);
    }
	//printf("%lld\n",time);
    if(min > time){
      min = time;
    }
  }

  printf("%lld\n",min);

  return 0;
}