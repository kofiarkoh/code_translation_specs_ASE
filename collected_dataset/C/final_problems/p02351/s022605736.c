#include<stdio.h>
long long N[400010],L[100010],T;
long long  hyouka(int a,int b){return a+b;}
void lsset(int n,int *d){
  int i;
  for(T=1;T<n;T*=2);
  for(i=0;i<T;i++)N[T+i]=i<n?d[i]:0;
  for(i=T-1;i;i--)N[i]=hyouka(N[i*2],N[i*2+1]);
}
void lasy(int i,int l,int r){
  if(L[i]==0)return;
  N[i]+=L[i]*(r-l);
  if(r-l>1){
    L[i*2  ]+=L[i];
    L[i*2+1]+=L[i];
  }
  L[i]=0;
}
void lsadd(int a,int b,int c,int i,int l,int r){//printf("%d %d %d\n",i,l,r);
  lasy(i,l,r);
  if(r<=a||b<=l)return;
  if(a<=l&&r<=b){
    L[i]+=c;
    lasy(i,l,r);
  }
  else{
    lsadd(a,b,c,i*2  ,l,(l+r)/2);
    lsadd(a,b,c,i*2+1,(l+r)/2,r);
    N[i]=hyouka(N[i*2],N[i*2+1]);
  }
}
long long lsget(int a,int b,int i,int l,int r){
  lasy(i,l,r);
  if(b<=l||r<=a)return 0;
  if(a<=l&&r<=b)return N[i];
  return hyouka(lsget(a,b,i*2,l,(l+r)/2),lsget(a,b,i*2+1,(l+r)/2,r));
}
int main(){
  int n,m,i,a,b,c,q,d[100010]={};
  scanf("%d %d",&n,&m);
  lsset(n,d);
  for(i=0;i<m;i++){
    scanf("%d %d %d",&q,&a,&b);
    if(q)printf("%lld\n",lsget(a-1,b,1,0,T));
    else{
      scanf("%d",&c);
      lsadd(a-1,b,c,1,0,T);
    }
  }
  return 0;
}