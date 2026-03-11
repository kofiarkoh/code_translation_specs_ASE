#include<stdio.h>
int R=1,C=1,H[2000010],N[2000010];
//ティツゥツ陛、ツセツ。テゥツ鳴「テヲツ閉ーテッツシツ暗」ツ??」ツ?セテ」ツ?ッMINテッツシツ?
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]<N[H[b]]?1:0;
}
//テヲツ個ソテ・ツ?・テゥツ鳴「テヲツ閉ー
void hin(int a){
  int i=C++;
  for(N[H[0]=R]=a;hyouka(0,i/2);i/=2)H[i]=H[i/2];
  H[i]=R++;
}
//テ・ツ渉姪」ツつ甘・ツ?コテ」ツ?凖ゥツ鳴「テヲツ閉ー
int hout(){
  int rt=H[1],i,j=2,k=H[--C];
  for(i=1;hyouka(i,C);i=j)H[i]=H[j=i*2+1-hyouka(i*2,i*2+1)];
  H[j/2]=k;
  return rt;
}
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
int Union(int a,int b){
  if(Find(a,b)){
    for(++a;p[a];a=p[a]);
    for(++b;p[b];b=p[b]);
    d[a]<d[b]?(p[a]=b):(p[b]=a);
    if(d[a]==d[b])d[a]++;
    return 1;
  }
  return 0;
}
int main(){
  int a[10010],b[10010],c,n,m,i,j;
  while(scanf("%d %d",&n,&m),n){
    for(i=0;i<n+2;i++)p[i]=d[i]=0;
    R=C=1;
    for(i=0;i<m;i++){
      scanf("%d %d %d",&a[R],&b[R],&c);
      hin(c);
    }
    for(i=0;i<n/2;i+=Union(a[j],b[j]))c=N[j=hout()];
    printf("%d\n",c);
  }
  return 0;
}