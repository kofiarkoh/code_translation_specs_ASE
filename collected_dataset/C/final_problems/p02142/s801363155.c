#include<stdio.h>
long long R=1,C=1,H[800010],N[800010];
//評価関数（いまはMIN）
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]<N[H[b]]?1:0;
}
//挿入関数
void hin(long long a){
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
double g(long long a,long long b,double c){return (b-1)/(c+a)+1+c;}
long long f(long long a,long long b){
  double l=0,r=1e16,m1,m2,i;
  //double m1,m2;
  while(l+0.0001<r){//printf("%d %d %d %d\n",l,m1,m2,r);
    m1=(l+l+r)/3;
    m2=(l+r+r)/3;//printf("%lf %lf %lf %lf\n",l,m1,m2,r);
    if(g(a,b,m1)>=g(a,b,m2))l=m1;//(long long)(m1+0.5);
    else                   r=m2;//(long long)(m2+0.5);
    //if(l+2>=r)break;
    //(b-1)/(m+a)>=(b-1)/(m+1+a)+1?(l=m):(r=m);
  }//printf("%lf %lld\n",l,(long long)(l+0.5));
  return (long long)(l+0.5);
}
int      ta[200010],to[400010],nt[400010];
long long a[400010], b[400010], c[400010],CO[200020];
void dijk(int v,int mi){
  int i,id[200010],f[200010]={0};
  R=C=1;
  for(i=0;i<=v;i++)CO[i]=1e18+10;
  CO[mi]=0;
  while(f[mi]-1){
    f[mi]=1;
    for(i=ta[mi];i+1;i=nt[i]){
      if(CO[mi]>c[i]){
	if(   CO[      to[i]]>CO[mi]+(b[i]-1)/(CO[mi]+a[i])+1){
	  hin(CO[id[R]=to[i]]=CO[mi]+(b[i]-1)/(CO[mi]+a[i])+1);
	}
      }
      else{
	if(   CO[      to[i]]>c[i]+  (b[i]-1)/(c [i ]+a[i])+1){
	  hin(CO[id[R]=to[i]]=c[i]+  (b[i]-1)/(c [i ]+a[i])+1);
	}
      }
    }
    while(f[mi]&&C-1)mi=id[hout()];
  }
}
int main(){
  int n,m,s,t,i,j,k;
  scanf("%d %d %d %d",&n,&m,&s,&t);
  for(i=0;i<=n;i++)ta[i]=-1;
  for(i=0;i<m;i++){
    scanf("%d %d %lld %lld",&j,&k,&a[i],&b[i]);
    to[i]=k;
    nt[i]=ta[j];
    ta[j]=i;
    to[i+m]=j;
    nt[i+m]=ta[k];
    ta[k  ]=i+m;
    a[i+m]=a[i];
    b[i+m]=b[i];
    c[i]=c[i+m]=f(a[i],b[i]);
  }
  //for(i=0;i<m;i++)printf("%d\n",c[i]);
  //printf("%d %d\n",(b[0]-1)/(c[0]+a[0])+1,c[0]+  (b[0]-1)/(c[0]+a[0])+1);
  dijk(n+1,s);
  printf("%lld\n",CO[t]<1e18?CO[t]:-1);
  return 0;
}

