#include<stdio.h>
#define MAX 1000001

int main(void)
{
  int a, d, n, sum,  count, k=0, p=0;
  int ans[500];
  int i, j, x[MAX];

  for(i=2; i<MAX; i++){
    x[i]=1;
  }
  for(i=2; i<1001; i++){
    if(x[i]==1){
      for(j=i*i; j<MAX; j+=i){ //?´???°??§??????????????°??????
//	if(j%i==0){
	  x[j]=0;
//	}
      }
    }
  }
  x[2]=1;
  while(1){
    scanf(" %d %d %d", &a, &d, &n);
    if(a==0&&d==0&&n==0){
      for(k=0; k<p; k++){
	printf("%d\n", ans[k]);
      }
      return 0;
    }
    else{
      sum=a;
      count=0;

      while(1){
	if(x[sum]==1){
	  count++;
	  if(count==n){
	    ans[p]=sum;
	    p++;
	    break;
	  }
	}
	sum+=d;
      }
    }
  }
  return 0;
}