#include<stdio.h>
#include<stdlib.h>

int main()
{
  int N;
  unsigned long long K;
  unsigned long long a[5000];
  int i;
  int j;
  unsigned long long maxsum=0;
  unsigned long long sets[5000];
  unsigned long long sum[5000];
  unsigned long long temp;
  unsigned long long necessary[5000];
  int necessarycount=0;
  int unnecessary=0;
  int arrsize=N*N;
  scanf("%d %llu",&N,&K);
  for(i=0; i<N;i++) 
  {
          scanf("%llu",&a[i]);
          maxsum+=a[i];
  }
  if(maxsum<K){  
          // all cards are unnecessary since cannot get full sum
          printf("%d\n",N);
  }else{
          if(maxsum==K){
                  // all cards are necessary since need them to get full sum
                  printf("0\n");
          }else{
                  // find possible subsets of cards
                  for(i=0; i<N; i++){
                          sum[i]=maxsum;
                          necessary[i]=0;
                          sets[i]=0;
                  }
                  for(i=0; i<N; i++){
                          for(j=0; j<N; j++) {
                                  temp = a[(i+j)%N];
                                  if((sum[i]-temp)>=K)
                                  {
                                          sum[i]=sum[i]-temp;
                                          sets[j]=0;
                                  }else{
                                          sets[j]=temp;
                                  }
                          }
                          for(j=0;j<N;j++){
                                  if(necessary[(i+j)%N]<sets[j])
                                  {
                                          necessary[(i+j)%N]=sets[j];
                                          necessarycount++;
                                  }
                                  sets[j]=0;
                          }
                  }
                  unnecessary=N-necessarycount;
                  printf("%d\n",unnecessary);
          }
  }
return 0;
}
