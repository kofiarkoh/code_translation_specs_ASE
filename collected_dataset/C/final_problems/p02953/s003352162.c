#include<stdio.h>
int main(void){
  int n;
  scanf("%d",&n);
  int a[n],i;
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
  }
 
  for(i=1;i<n;i++){
    if(a[i-1]-a[i]==1){
      a[i-1]-=1;
    }
  }
  for(i=1;i<n;i++){
    if(a[i-1]>a[i]||a[i-1]-a[i]>=2){
      printf("No");
    return 0;
    }
  }
  printf("Yes");
  return 0;
}
