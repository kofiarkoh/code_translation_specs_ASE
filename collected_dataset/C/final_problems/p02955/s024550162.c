#include <stdio.h>
#include <stdlib.h>

#define swap(x,y) do{long t=x; x=y; y=t;}while(0)

void sort(long a[],int left,int right){
	int pl=left;
	int pr=right;
	long x=a[(pl+pr)/2];

	do{
		while(a[pl]<x) pl++;
		while(a[pr]>x) pr--;
		if(pl<=pr){
			swap(a[pl],a[pr]);
			pl++;
			pr--;
		}
	}while(pl<=pr);
	if(left<pr) sort(a,left,pr);
	if(pl<right) sort(a,pl,right);
}

int main(void){
  int i,j;

  int n;
  long k;
  scanf("%d%ld",&n,&k);

  long long all_sum=0;
  long a[n];
  for(i=0;i<n;i++){
    scanf("%ld",&a[i]);
    all_sum+=a[i];
  }

  sort(a,0,n-1);

  int rest;
  long long sum=0;
  for(i=0;;i++){
    if(sum+a[i]<k){
      sum+=a[i];
    }else{
      rest=n-i;
      break;
    }
  }

  if(all_sum<k){
    printf("%lld\n",all_sum);
    exit(0);
  }

  if(a[0]>k){
    for(i=k;i>=-k;i--){
      if(all_sum%(a[0]+i)==0){
        printf("%ld\n",a[0]+i);
        exit(0);
      }
    }
  }

  for(i=rest;;i++){
    if(all_sum%i==0){
      printf("%lld\n",all_sum/i);
      break;
    }
  }

  return 0;
}
