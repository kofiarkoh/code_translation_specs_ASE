#include<stdio.h>

int prime(int n){
  int i;
  
  if((n!=2 && n%2==0)||n==1){
    return 0;
  }
  for(i=3;i<n;i++){
    if(n%i==0){
      return 0;
    }
    if(n/2<i){
      break;
    }
  }
  return 1;
}

int main(){
  int n,i,r;
  int a[10000];
  int b[10000];
  int c[10000];
  
  while(scanf("%d",&n)!=EOF){
    for(i=1;i<=n;i++){
      a[i-1]=i;
      b[i-1]=n-i+1;
      c[i-1]=0;
    }
    for(i=0;i<n;i++){
      c[i]=c[i]+prime(a[i]);
      c[i]=c[i]+prime(b[i]);
    }
    r=0;
    for(i=0;i<n;i++){
      if(c[i]==2){
	r++;
      }
    }
    printf("%d\n",r);
  }
  return 0;
}