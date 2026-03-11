#include<stdio.h>

typedef struct {
  int dis;
  int wait;
  int kan;
} unko;

int kiriage( double x )
{
    return (int)( x < 0.0 ? x : x+0.9 );
}

int main(){
  int i,j,k,N,time=0;
  scanf("%d",&N);
  unko A[N];
  for(i=0;i<N-1;i++){
    scanf("%d %d %d",&A[i].dis,&A[i].wait,&A[i].kan);
  }
  for(i=0;i<N-1;i++){
    time =0;
    for(j=i;j<N-1;j++){
      if(A[j].wait >= time){
        time+=(A[j].wait-time)+A[j].dis;
      }
      else{
        time+= (kiriage((double)(time/A[j].kan))*A[j].kan-time)+A[j].dis;
      }
    }
    printf("%d\n",time);
  }
  puts("0");
  return 0;
}
  