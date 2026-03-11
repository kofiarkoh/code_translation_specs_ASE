#include<stdio.h>

int main()
{
  int n;
  int a[101];
  int i,j;
  int A,B;
  int t;
  A=0;
  B=0;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
  }
  for(i=0;i<n;i++){
    for(j=i+1;j<n;j++){
      t=a[i];
      if(a[i]<a[j]){
        a[i]=a[j];
        a[j]=t;
      }
    }
  }
  for(i=0;i<n;i++){
    if(i%2==0) A+=a[i];
    if(i%2==1) B+=a[i];
  }
  printf("%d",A-B);
  return 0;
}
