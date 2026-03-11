#include <stdio.h>
#include <stdlib.h>
int main (){
  int N,match=0;
  scanf("%d",&N);
  int a[N],b[N],x,y;
  for(int i=0;i<N;i++){
    scanf("%d",&a[i]);
    b[i]=0;
  }
  for(int i=0;i<N;i++){
    x=a[i]-1;
    b[x]++;
  }
  
  for(int i=0;i<N;i++){
    match=match+( b[i]*(b[i]-1) )/2;
  }
  for(int i=0;i<N;i++){
    x=a[i]-1;
    y=( b[x]*(b[x]-1) )/2-( (b[x]-1)*(b[x]-2) )/2;
    printf("%d\n",match-y);
  }
  return 0;
}
