#include <stdio.h>

int main(){
  int n,k,keta=0,flag,a=1,b;
  scanf("%d %d",&n,&k);
  
  int d[k+1];
  for(int i=1;i<=k;i++){
    scanf("%d",&d[i]);
  }
  
  for(int i=1;i>0;i++){
    a=a*10;
    keta+=1;
    if(a>=n){
      if(a>n){
        a=a/10;
        break;
      }
      if(a==n){
        keta+=1;
        break;
      }
    }
  }
  
  for(int i=n;i>0;i++){
    int ii=i;
    flag=0;
    if(a==i){
      keta+=1;
      a=a*10;
    }
    
    b=a;
    for(int j=keta;j>=1;j--){
      for(int l=1;l<=k;l++){
        if((ii-(ii%b))/b==d[l]){
          flag=1;
          break;
        }else{
          ii=ii%b;
        }
      }
      b=b/10;
      if(flag==1){
        break;
      }
    }
    
    if(flag==0){
      printf("%d",i);
      break;
    }
  }
  
  return 0;
}