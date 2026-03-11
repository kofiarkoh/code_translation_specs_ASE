#include "stdio.h"

long long int my_f(long long int N){
  if (N%2==0) {
    return N/2;
  }
  return 3*N+1;
}

int main(int argc, char const *argv[]) {
  long long int S;
  int c_f=0;
  int c_t=0;
  scanf("%lld", &S);
  int i=1;
  while (S!=1) {
    S=my_f(S);
    if (S==2) {c_t=1;}
    if (S==4) {c_f=1;}
    i++;
  }
  if (c_f==1) {
    i=i+1;
  }else if(c_t==-1){
    i=i+2;
  }else{
    i=i+3;
  }
  printf("%d", i);
  return 0;
}
