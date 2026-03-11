#include<stdio.h>
int p[100010]={0};
int d[100010]={0};
int Find(int a,int b){
  int r,n;
  for(r=++a;p[r];r=p[r]);
  for(;n=p[a];a=n)p[a]=r;
  for(r=++b;p[r];r=p[r]);
  for(;n=p[b];b=n)p[b]=r;
  return a-b?1:0;
}
void Union(int a,int b){
  if(Find(a,b)){
    for(++a;p[a];a=p[a]);
    for(++b;p[b];b=p[b]);
    d[a]<d[b]?(p[a]=b):(p[b]=a);
    if(d[a]==d[b])d[a]++;
  }
}
int main(){
  int n,m,t[100010],l,k,i;
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++){
    scanf("%d %d",&k,&t[i]);
    while(--k){
      scanf("%d",&l);
      Union(t[i],l);
    }
  }
  //for(i=1;i<=m;i++)printf("%d %d\n",i,p[i]);
  for(i=1;i<n;i++){
    if(Find(t[0],t[i]))break;
  }
  printf("%s\n",i==n?"YES":"NO");
  return 0;
}
    
      
