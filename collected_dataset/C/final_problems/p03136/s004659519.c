#include<stdio.h>
int main(void){
  int n,i,j,tmp,sum=0;
  scanf("%d",&n);
  int a[n];
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
  }
  for(i=0;i<n;i++){
    for(j=i+1;j<n;j++){
      if(a[i]>a[j]){
        tmp=a[i];
        a[i]=a[j];
        a[j]=tmp;
      }
    }
  }
  for(i=0;i<n-1;i++){
    sum+=a[i];
  }
  if(a[i]<sum){
    printf("Yes");
  }else{
    printf("No");
  }
  return 0;
}