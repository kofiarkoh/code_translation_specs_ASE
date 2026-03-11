#include<stdio.h>
int main(){
  int n;
  scanf("%d",&n);
  long long int l[n];
  l[0] = 2,l[1] = 1;
  for(int i = 1;i < n;i++){
    l[i + 1] = l[i] + l[i - 1];
  }
  printf("%lld",l[n]);
  return 0;
}