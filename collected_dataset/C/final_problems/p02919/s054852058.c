
#include <stdio.h>
#include <stdlib.h>

unsigned long long getSum(int *P, int N, int head, int tail){
  int max,second,max_i,second_i,i;
  if(head>=tail)
    return 0;
  if(P[head]>P[head+1]){
    max = P[head];
    second = P[head+1];
    max_i = head;
    second_i = head+1;
  }
  else{
    max = P[head+1];
    second = P[head];
    max_i = head+1;
    second_i = head;
  }
  for(i=head+2;i<=tail;i++){
    if(P[i]>max){
      second = max;
      max = P[i];
      second_i = max_i;
      max_i = i;
    }
    else if(P[i]>second){
      second = P[i];
      second_i = i;
    }
  }
  int smaller,larger;
  if(max_i<second_i){
    smaller = max_i;
    larger = second_i;
  }
  else{
    smaller = second_i;
    larger = max_i;
  }
  unsigned long long left,right,inter;
  left = getSum(P, N, head, larger-1);
  right = getSum(P, N, smaller+1, tail);
  inter = getSum(P, N, smaller+1, larger-1);
  
  //printf("%d %d %d*%d*%d=%d\n",head,tail,second,smaller-head+1,tail-larger+1,second * (smaller-head+1) * (tail-larger+1));
  
  return left + right - inter + (unsigned long long)second * (smaller-head+1) * (tail-larger+1);
}


int main(){
  unsigned long long sum=0;
  int N;
  int *P,i,j;
  scanf("%d",&N);
  P = (int*)malloc(N*sizeof(int));
  for(i=0;i<N;i++)
    scanf("%d",&(P[i]));
  
  sum = getSum(P,N,0,N-1);
  
#if 0
  /*** O(N^2)-time ***/
  for(i=0;i<N-1;i++){
    // <i,i+1>
    int max,second;
    if(P[i+1]>P[i]){
      max = P[i+1];
      second = P[i];
    }
    else{
      max = P[i];
      second = P[i+1];
    }
    sum += (unsigned long long)second;
    // <i,j>
    for(j=i+2;j<N;j++){
      if(P[j]>max){
	second = max;
	max = P[j];
      }
      else if(P[j]>second)
	second = P[j];
      sum += (unsigned long long)second;
    }
  }
#endif		 

  printf("%llu\n",sum);
  return 0;
}
