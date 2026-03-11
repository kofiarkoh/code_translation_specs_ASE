#include<stdio.h>
int lub(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lub(l,m,n,d):lub(m+1,r,n,d);
}
int R=1,C=1,H[2000010],N[2000010];
//評価関数（いまはMAX）
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]<N[H[b]]?1:0;
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
long long int MIN(long long a,long long b){return a<b?a:b;}
/*long long f(long long a,long long b,long long min,long long *t){
  long long i,ans=min;
  for(i=0;i<b;i++)ans+=MIN(min+t[i],t[i]*(a+1));//,printf("%lld\n",ans);
  return ans;
  }//*/
long long dx[100010],dy[100010];
long long f(int a,int b,long long min,int *s,int *t,long long *d){
  long long i,j;
  //for(i=0;i<a;i++)printf("%d ",t[i]);printf("\n"); 
  for(i=0;i<b;i++){
    j=lub(0,a,s[i],t);//printf("%d %lld\n",s[i],j);
    min+=d[j]+s[i]*(a+1-j);
  }
  return min;
}
    
int main(){
  int i,h,w,sw,sh,a,x[100010],y[100010];
  scanf("%d %d",&w,&h);
  for(i=sw=0;i<w;i++){
    scanf("%d",&a);
    hin(a);
    sw+=a;
  }
  for(i=0;i<w;i++)x[i]=N[hout()];
  for(i=dx[0]=0;i<w;i++)dx[i+1]=dx[i]+x[i];
  R=C=1;
  for(i=sh=0;i<h;i++){
    scanf("%d",&a);
    hin(a);
    sh+=a;
  }
  for(i=0;i<w;i++)y[i]=N[hout()];
  for(i=dy[0]=0;i<w;i++)dy[i+1]=dy[i]+y[i];
  printf("%lld\n",sh<sw?f(h,w,sh,x,y,dy):f(w,h,sw,y,x,dx));
  return 0;
}
