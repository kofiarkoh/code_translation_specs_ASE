#include<stdio.h>
#include<stdlib.h>
#include<math.h>

int max(int a, int b){
  return a >= b ? a : b;
}
int min(int a, int b){
  return b >= a ? a : b;
}
int factorial(int n) {
    if (n > 0) return n*factorial(n - 1);
		else return 1;
}
int compare_up_int(const void *a, const void *b){
   return *(int*)a - *(int*)b;
}
int compare_down_int(const void *a, const void *b){
   return *(int*)b - *(int*)a;
}
int euclid(int a, int b){
  if(a < b){int tmp = a;a = b;b = tmp;}
  int r = a % b;
  if(r < 0) r += b;
  while(r != 0){
    a = b;b = r;r = a % b;
    if(r < 0) r += b;
  }
  return b;
}

int main(){
  int N,ans=1145141,count;
  scanf("%d",&N);
  char we[N];
  scanf("%s",we);
  for(int i=0;i<N;i++){
    count = 0;
    for(int j=0;j<i;j++){
      if(we[j] != 'E')count++;
    }
    for(int j=i+1;j<N;j++){
      if(we[j] != 'W')count++;
    }
    ans = min(ans,count);
  }
  printf("%d",ans);
  return 0;
}
