#include<stdio.h>

int main(){
  int N,n,a[200000],max;
  
  scanf("%d",&n);
  for(int i=0;i<n;i++){
    scanf("%d",&a[i]);
  }
  for(int i=0;i<n;i++){ 
    max=0;
    for(int j=0;j<n;j++){
      if(i == j){
        j++;
      }
    if(max<a[j]){
      max=a[j];
    }
    }
    printf("%d\n",max);
  }
  return 0;
}