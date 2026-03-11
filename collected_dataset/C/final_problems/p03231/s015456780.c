#include <stdio.h>
#include <string.h>

int gcd(int a, int b) {
  if (b == 0){
    return a;
  }
  return gcd(b, a % b);
}

int lcm(int a, int b) {
  return a * b / gcd(a, b);
}



int main(){
  int N,M;
  scanf("%d%d",&N,&M);
  char S[N];
  char T[M];
  scanf("%s",S);
  scanf("%s",T);
  int i;
  int flag=0;
  int ans=-1;
  if(S[0]!=T[0]){
    flag=1;
  }
  for(i=1;i<N;i++){
    if(i*M%N==0){
      if(S[i]!=T[i*M/N]){
	flag=1;
      }
    }
  }
  if(flag==0){
    ans=lcm(N,M);
  }
  printf("%d",ans);
  return 0;
}

    
