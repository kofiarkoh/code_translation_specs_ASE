#include<stdio.h>

int main(){
  long long int c,x[100001],v[100000],a[100000][2]={},b[100000][2]={},am[100000]={},bm[100000]={},max=0;
  int n,i,ap[100000],bp[100000],p;
  scanf("%d %ld",&n,&c);
  for(i=0;i<n;i++){
    scanf("%ld %ld",&x[i],&v[i]);
  }
  ap[0]=bp[n-1]=100000;
  x[100000]=0;
  a[0][0]=v[0]-x[0];
  a[0][1]=a[0][0]-x[0];
  if(a[0]>0){
    am[0]=a[0][0];
    ap[0]=0;
  }
  for(i=1;i<n;i++){
    a[i][0]=a[i-1][0]+v[i]-x[i]+x[i-1];
    a[i][1]=a[i][0]-x[i];
    if(am[i-1]<a[i][0]){
      am[i]=a[i][0];
      ap[i]=i;
    }
    else{
      am[i]=am[i-1];
      ap[i]=ap[i-1];
    }
  }
  b[n-1][0]=v[n-1]-c+x[n-1];
  b[n-1][1]=b[n-1][0]-c+x[n-1];
  if(b[n-1][0]>0){
    bm[n-1]=b[n-1][0];
    bp[n-1]=n-1;
  }
  for(i=n-2;i>=0;i--){
    b[i][0]=b[i+1][0]+v[i]+x[i]-x[i+1];
    b[i][1]=b[i][0]-c+x[i];
    if(bm[i+1]<b[i][0]){
      bm[i]=b[i][0];
      bp[i]=i;
    }
    else{
      bm[i]=bm[i+1];
      bp[i]=bp[i+1];
    }
  }
  if(max<b[0][0]){
    max=b[0][0];
    p=0;
  }
  for(i=1;i<n;i++){
    if(max<b[i][0]+am[i-1]){
      max=b[i][0]+am[i-1];
      p=i;
    }
  }
  if(max<am[n-1]){
    p=n;
    max=am[n-1];
  }
  if(p!=n&&p!=0){
    if(c-x[p]<x[ap[p-1]]) max-=c-x[p];
    else max-=x[ap[p-1]];
  }
  printf("%ld\n",max);
}