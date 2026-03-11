#include<stdio.h>

int main(){
  int n,m;
  int all=0;
  int ans=0;
  
  scanf("%d %d",&n,&m);
  
  int l[n];
  for(int i=0;i<n;i++){
    scanf("%d",&l[i]);
    all+=l[i];
  }
  
  for(int i=0;i<n;i++){
    
    if(l[i]>=all/(4*m)){
      ans++;
    }
  }
    if(ans>=m){
      printf("Yes");
    }else{
      printf("No");
    }
  
  return 0;
}
  
  
    
  