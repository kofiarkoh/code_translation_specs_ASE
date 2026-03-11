#include<stdio.h>
int MIN(int a,int b){return a<b?a:b;}
//添字ヒープを使うための関数達!
int R=1,C=1,H[2000010],N[2000010];
//評価関数（いまはMAX）
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]>N[H[b]]?1:0;
}
//挿入関数
void hin(int a){
  int i=C++;
  for(N[H[0]=R]=a;hyouka(0,i/2);i/=2)H[i]=H[i/2];
  H[i]=R++;
}
//取り出す関数
int hout(){
  int rt=H[1],i,j=2,k=H[--C];
  for(i=1;hyouka(i,C);i=j)H[i]=H[j=i*2+1-hyouka(i*2,i*2+1)];
  H[j/2]=k;
  return rt;
}
int n,m;
int main(){
  int i,s,a,b,c[100010]={};
  int p[100010]={};
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++){
    scanf("%d",&a);
    hin(a);
  }
  b=-1;
  for(i=0;i<n;i++){
    a=N[hout()];
    if(b==a){
      p[a%m]++;
      b=-1;
    }
    else b=a;
    c[a%m]++;
  }
  //for(i=0;i<m;i++)printf("%d %d %d\n",i,c[i],p[i]);
  s=c[0]/2;
  if(m%2==0)s+=c[m/2]/2;
  for(i=1;i*2<m;i++){
    if(c[i]>c[(m-i)%m]){
      //printf("%d %d %d %d\n",c[i],c[m-i],p[i],p[m-i]);
      a=MIN((c[i]-c[m-i])/2,p[i]);
      c[i]-=a*2;
    }
    else{
      a=MIN((c[m-i]-c[i])/2,p[m-i]);
      c[m-i]-=a*2;
    }
    s+=MIN(c[i],c[m-i]);//printf("%d ",s);
    s+=a;//printf("%d\n",s);
  }
  printf("%d\n",s);
  return 0;
}
