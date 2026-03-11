#include<stdio.h>
void push(long int x[],long int i,long int k){//最小が根になる x: 配列，i:push後のデータ数，k: pushするデータ
  x[i]=k;
  for(int j=i;j>1;j/=2)if(x[j]<x[j/2])x[j]^=x[j/2]^=x[j]^=x[j/2];
}
long int pop(long int x[],long int i){//最初値を取り出す x: 配列，i:pop前のデータ数
  int j=1,k;
  x[0]=x[1];x[1]=x[i];
  while(j*2<i&&(x[j*2]<x[j]||x[j*2+1]<x[j])){
    k=x[j*2]<x[j*2+1]?j*2:j*2+1;
    x[j]^=x[k]^=x[j]^=x[k];
    j=k;
  }
  return x[0];
}
int main(){
  long int a[300000],x[100001],y[100001],k[100001],l[100001];
  long int i,n,j;
  scanf("%ld",&n);
  for(i=0;i<n*3;i++)scanf("%ld",&a[i]);
  k[0]=0;
  for(i=0;i<n;i++){push(x,i+1,a[i]);k[0]+=a[i];}
  for(i=0;i<n;i++){k[i+1]=k[i]+a[n+i]-pop(x,n);push(x,n,a[n+i]);}
  k[n]=0;
  for(i=0;i<n;i++){push(y,i+1,-a[n*3-i-1]);l[n]-=a[n*3-i-1];}
  for(i=0;i<n;i++){l[n-i-1]=l[n-i]-a[n*2-i-1]-pop(y,n);;push(y,n,a[n*2-i-1]);}
  j=k[0]+l[0];
  for(i=0;i<n;i++)j=j<k[i+1]+l[i+1]?k[i+1]+l[i+1]:j;
  printf("%ld",j);
}
