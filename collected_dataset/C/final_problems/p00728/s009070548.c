#include <stdio.h>
int main (){
  int n,a[20],i,k,t,sum,Sum,min,max;
  
  while (1){
    scanf("%d",&n);
    if(n==0)break;
    sum=0;
    for (i=0;i<n;i++){
      scanf("%d",&a[i]);
      sum=sum+a[i];
    }
    if(a[0]>a[1]){
      k=a[0];
      a[0]=a[1];
      a[1]=k;
    }
    max=a[1];
    for (i=1;i<n;i++){
      if(max<a[i]){
	k=max;
	max=a[i];
	a[i]=k;
      }
    }
    if (a[n-1]<a[n-2]){
      k=a[n-1];
      a[n-1]=a[n-2];
      a[n-2]=k;
    }
    min=a[n-2];
    for (i=n-2;i>=0;i--){
      if(min>a[i]){
	k=min;
	min=a[i];
	a[i]=k;
      }
    }
    Sum=sum-(min+max);
      printf("%d\n",Sum/(n-2));
  }
  
  return 0;
}
      