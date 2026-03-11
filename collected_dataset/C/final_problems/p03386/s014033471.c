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
  int K;
  long long A,B;
  
  int number;
//  long long N[101]={};
  int i;
//    int k=0;
  scanf("%lld", &A);
  scanf("%lld", &B);
  scanf("%d", &K);

  for(i=0;i<K;i++){
//      if(B<=i+1) return 0;
    printf("%lld\n", A+i);
      if(B-A<=i) return 0;
    
  }

  for(i=0;i<K;i++){
    if((B+i+1)>=A+(2*K))  printf("%lld\n", B+i-K+1);
  }
//  if(ans[0]&&ans[1]&&ans[2])  printf("Yes\n");  
//  else printf("No\n"); 
return 0;
}
