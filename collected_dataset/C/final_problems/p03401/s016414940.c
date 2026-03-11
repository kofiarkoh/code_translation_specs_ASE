#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
 
int compare_int(const void *a, const void *b)
{
    return *(int*)a - *(int*)b;
}
 
int main()
{
  int N;
  int A[100020] = {};
  int i,j;
  int number;
  char s[10];
  int  cnt_sum=0;
  int  cnt=0;
  int  cnt_a=0;
  int  cnt_b=0;
  int  cnt_c=0;
  int  ans=0;

//    int k=0;
  scanf("%d", &N);
//  scanf("%lld %lld",  &D, &X);
  A[0]=0;
  A[N+1]=0;
  for(i=1;i<N+2;i++){
    scanf("%d", &A[i]);
//    if(A[i-1]<A[i]) cnt=(long long)abs(A[i]-A[i-1]);
    cnt=abs(A[i]-A[i-1]);
    //    else cnt=abs(A[i-1]-A[i]);
 //   printf("%lld\n",  cnt);
    cnt_sum = cnt_sum + cnt ;
  }
  for(i=1;i<N+1;i++){
//    if(A[i-1]<A[i+1]) cnt_a=(long long)abs(A[i+1]-A[i-1]);
//    else            cnt_a=(long long)abs(A[i-1]-A[i+1]);  
//    if(A[i-1]<A[i]) cnt_b=(long long)abs(A[i]-A[i-1]);
//    else            cnt_b=(long long)abs(A[i-1]-A[i]);
//    if(A[i]<A[i+1]) cnt_c=(long long)abs(A[i+1]-A[i]);
//    
       cnt = cnt_sum + abs(A[i-1]-A[i+1]) - abs(A[i]-A[i-1])- abs(A[i+1]-A[i]);

    printf("%d\n",  cnt);  
  }

return 0;
}
