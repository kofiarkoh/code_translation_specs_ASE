#include<stdio.h>
int S[4*300010];
int T,NN=0;
//評価関数（今はMIN）
int hyoka(int a,int b){return a>b?a:b;}
//セグ木を長さnの数列dで初期化、構成
void sset(int n){
  int i;
  for(T=1;T<n;T*=2);
  for(i=0;i<T*2;i++)S[i]=NN;
}
//セグ木のupdate関数
//i番目の数値をnに変更、再構成
void sud(int i,int n){for(S[i+=T]=n;i/=2;)S[i]=hyoka(S[i*2],S[i*2+1]);}
//セグ木のfind関数
//[a,b)区間の呼び出しはsout(a,b,1,0,T);
int sout(int a,int b,int i,int l,int r){
  if(r<=a||b<=l)return NN;
  if(a<=l&&r<=b)return S[i];
  return hyoka(sout(a,b,i*2,l,(l+r)/2),sout(a,b,i*2+1,(l+r)/2,r));
}
int sget(int a,int b){return sout(a,b,1,0,T);}
int MAX(int a,int b){return a<b?b:a;}
int MIN(int a,int b){return a<b?a:b;}
int main(){
  int n,m,i,a,b,ans=0,l,r;
  scanf("%d %d",&n,&m);
  sset(n);
  for(i=0;i<n;i++){
    scanf("%d",&a);
    a--;
    l=MAX(0,a-m);
    r=MIN(T,a+m+1);
    b=sget(l,r);
    sud(a,b+1);
    ans=MAX(ans,b+1);//printf("%d %d\n",a+1,ans);
  }
  printf("%d\n",ans);
  return 0;
}
