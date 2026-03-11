#include<stdio.h>

int main(void){
  long long int N,A,B;
  long long int i;
  long long int h[100000];
  long long int K;
  long long int left,right;
  long long int sum;

  scanf("%lld %lld %lld",&N,&A,&B);
  //printf("%d\n", N);

  for(i=0;i<N;++i){
    scanf("%llu",&h[i]);
  }

  left=0;
  right=1000000000;

  while(left!=right){
    K=(right+left)/2;
    sum=0;
    for(i=0;i<N;i++){
      if(h[i]-K*B > 0){
        // sum += (h[i]-K*B)%(A-B) ? (h[i]-B*K)/(A-B)+1 : (h[i]-B*K)/(A-B);
        if((h[i]-K*B)%(A-B)==0){
          sum += (h[i]-B*K)/(A-B);
        }else{
          sum += (h[i]-B*K)/(A-B)+1;
        }
      }
    }
    if(sum <= K){
      right=K;
    }else{
      left=K+1;
    }
  }

  printf("%lld\n", left);

  return 0;
}
