#include <stdio.h>

int main(){
  int N;
  scanf("%d",&N);
  if(N<600) printf("8\n");
  else if(N<800) printf("7\n");
  else if(N<1000) printf("6\n");
  else if(N<1200) printf("5\n");
  else if(N<1400) printf("4\n");
  else if(N<1600) printf("3\n");
  else if(N<1800) printf("2\n");
  else if(N<2000) printf("1\n");
  return 0;
}