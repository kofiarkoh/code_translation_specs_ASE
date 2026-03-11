#include <stdio.h>
#include <stdlib.h>

long long min(long long a,long long b){
  if(a < b){
    return a;
  }
  return b;
}

long long max(long long a,long long b){
  if(a > b){
    return a;
  }
  return b;
}

int binary_search(const long long *sum,const int ll,const int rr){
  long long median = (sum[rr]-sum[ll-1])/2;
  int left = ll;
  int right = rr;
  while(right-left > 1){
    int middle = (left+right)/2;
    if(sum[middle]-sum[ll-1] <= median){
      left = middle;
    }else{
      right = middle;
    }
  }

  long long x = llabs(sum[left]-sum[ll-1]-(sum[rr]-sum[left]));
  long long y = llabs(sum[right]-sum[ll-1]-(sum[rr]-sum[right]));
  return x < y ? left:right;
}


int main(void){
  int N;
  scanf("%d",&N);

  int A[N+1];
  long long sum[N+1];
  for(int i = 1;i <= N;i++){
    scanf("%d",&A[i]);
  }
  sum[0] = 0;
  for(int i = 1;i <= N;i++){
    sum[i] = sum[i-1]+A[i];
  }

  long long ans = sum[N];
  long long P,Q,R,S;
  for(int i = 2;i < N;i++){
    int left = binary_search(sum,1,i);
    int right = binary_search(sum,i+1,N);
    P = sum[left]-sum[0];
    Q = sum[i]-sum[left];
    R = sum[right]-sum[i];
    S = sum[N]-sum[right];
    ans = min(ans,max(P,max(Q,max(R,S)))-min(P,min(Q,(min(R,S)))));
  }

  printf("%lld\n",ans);

  return 0;
}