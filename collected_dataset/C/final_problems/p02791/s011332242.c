#include<stdio.h>

int main(){
  int n;
  scanf("%d",&n);
  int a[n],min=200001,count=0,i;
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    if(a[i]<min){
      min=a[i];
      count++;
    }
  }
  printf("%d",count);
  return 0;
}
