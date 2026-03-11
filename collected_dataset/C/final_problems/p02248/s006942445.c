#include <stdio.h>
#include <string.h>
#define N 10001
#define N2 1000001
int main(){
  char T[N2],P[N];
  int i,j,n,m,flag;
  
  scanf("%s",T);
  scanf("%s",P);
  
  n=strlen(T);
  m=strlen(P);
  
  for(i=0;i<n;i++) if(strncmp(&T[i],P,m)==0) printf("%d\n",i);

  return 0;
}