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
int main(){
  int n,m,a[100010],b[100010],t;
  while(scanf("%d %d",&n,&m),n){
    R=C=1;
    int d[100010]={0,1};
    while(m--){
      scanf("%d %d %d",&t,&a[R],&b[R]);
      hin(t);
    }
    while(C-1){
      t=hout();//printf("%d %d %d\n",d[a[t]],d[b[t]],d[a[t]]|d[b[t]]);
      d[b[t]]|=d[a[t]];
    }
    for(m=t=0;m<n;m++)t+=d[m+1];
    printf("%d\n",t);
  }
  return 0;
}