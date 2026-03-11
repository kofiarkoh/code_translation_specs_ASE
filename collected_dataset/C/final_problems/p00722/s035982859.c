#include<stdio.h>

int sosu(int n){
  if(n==1)return 0;
  int i;
  for(i=2;i<n;i++){
    if(n%i==0)return 0;
  }
  return 1;
}

int dirichlet(int a,int d,int n){
  int count = 0;
  while(1){
    if(sosu(a)==1)count++;
    if(count==n)return a;
    a = a + d;
  }
}

int main(void){
  int a[64],d[64],n[64];
  int i;
  int count;
  
  for(i=0;i>=0;i++){
    scanf("%d%d%d",&a[i],&d[i],&n[i]);
    if(a[i]==0&&d[i]==0&&n[i]==0)break;
  }

  count = i;

  for(i=0;i<count;i++){
    printf("%d\n",dirichlet(a[i],d[i],n[i]));
  }
  return 0;
}