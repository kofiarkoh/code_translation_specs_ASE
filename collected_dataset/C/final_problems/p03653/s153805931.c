#include<stdio.h>
#define int long long
int MIN(int a,int b){return a<b?a:b;}
int R=1,C=1,H[2000010],N[2000010];
//評価関数（いまはMIN）
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
int CO[100010];
int ta[100010],to[800010],co[800010],nt[800010],ca[800010],u[100010];
int fe[800010],fv[100010];
int mifl(int v,int e,int f,int s,int t,int *a,int *b,int *c,int *d){
  int i,j,k,min,mi,ans=0,id[100010];
  int M=1e9,h[100010]={0};
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    ca[i  ]=c[i];
    ca[i+e]=0;
    co[ta[a[i]]=i  ]= d[i];
    co[ta[b[i]]=i+e]=-d[i];
  }
  while(f>0){
    for(i=0;i<v;i++)CO[i]=M;
    for(i=0;i<v;i++)u[i]=0;
    R=C=1;
    fv[s]=fe[s]=-1;
    CO[mi=s]=0;
    while(u[mi]-1){
      u[mi]=1;
      for(i=ta[mi];i+1;i=nt[i]){
	if(ca[i]&&  CO[to[i]]>CO[mi]+co[          i]+h[          mi]-h[to[i]]){
	  hin(CO[id[R]=to[i]]=CO[mi]+co[fe[to[i]]=i]+h[fv[to[i]]=mi]-h[to[i]]);
	}
      }
      while(u[mi]&&C-1)mi=id[hout()];
    }
    if(CO[t]==M)return -1;
    for(i=0;i<v;i++)h[i]+=CO[i];
    min=f;
    for(i=t;i-s;i=fv[i])min=MIN(min,ca[fe[i]]);
    f-=min;
    ans+=h[t]*min;//printf("%lld %lld %lld\n",ans,h[t],min);
    for(i=t;i-s;i=fv[i]){
      ca[fe[i]]-=min;
      ca[(fe[i]+e)%(2*e)]+=min;
    }
  }
  return ans;
}
int n,m[5],i,j,k,a[400010],b[400010],c[400010],d[400010],r=0;
signed main(){
  long long M=1e9;
  scanf("%lld %lld %lld",&m[0],&m[1],&m[2]);
  n=m[0]+m[1]+m[2];
  for(i=0;i<n;i++){
    for(j=0;j<3;j++){
      scanf("%lld",&k);
      a[r  ]=n+j;
      b[r  ]=i;
      c[r  ]=1;
      d[r++]=M-k;
    }
    a[r  ]=i;
    b[r  ]=n+4;
    c[r  ]=1;
    d[r++]=0;
  }
  for(i=0;i<3;i++){
    a[r  ]=n+3;
    b[r  ]=n+i;
    c[r  ]=m[i];
    d[r++]=0;
  }//printf("\n");
  printf("%lld\n",M*n-mifl(n+5,r,n,n+3,n+4,a,b,c,d));
  return 0;
}
  
  
