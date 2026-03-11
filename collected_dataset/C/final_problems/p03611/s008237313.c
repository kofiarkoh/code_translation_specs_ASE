#include<stdio.h>
#include<stdlib.h>
int cmp(const void *a,const void *b){
  return *(int*)a - *(int*)b;
}

int main(){
  int n = 0;
  int a[100000] = {};
  int b[100000] = {};
  int i = 0;
  long max = 0;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    b[a[i]]++;
  }
  for(i=0;i<100000;i++){
    if(i != 0 && i != 99999){
      if(b[i-1]+b[i]+b[i+1] > max){
        max = b[i-1]+b[i]+b[i+1];
        //printf("%d/",b[i-1]+b[i]+b[i+1]);
      }
      
    }else if(i == 0){
      if(b[0] + b[1] > max){
        max = b[0] + b[1];
      }
    }else if(i == 99999){
      if(b[99998]+b[99999] > max){
        max = b[99999]+b[99998] ;
      }
    }
  }
  printf("%ld",max);
    
  /*for(i=0;i<n-1;i++){
    printf("%d ",b[i]);
  }*/
  return 0;
}