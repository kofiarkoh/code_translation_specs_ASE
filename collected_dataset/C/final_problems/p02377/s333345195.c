#include<stdio.h>
#define M 1000000000
int CO[100010];
int MIN(int a,int b){return a<b?a:b;}
int R=1,C=1,H[2000010],N[2000010];
int hyouka(int a,int b){
  if(C<b)return 1;
  if(C<a||b==0)return 0;
  return N[H[a]]<N[H[b]]?1:0;
}
void hin(int a){
  int i=C++;
  for(N[H[0]=R]=a;hyouka(0,i/2);i/=2)H[i]=H[i/2];
  H[i]=R++;
}
int hout(){
  int rt=H[1],i,j=2,k=H[--C];
  for(i=1;hyouka(i,C);i=j)H[i]=H[j=i*2+1-hyouka(i*2,i*2+1)];
  H[j/2]=k;
  return rt;
}
int mifl(int v,int e,int f,int s,int t,int *a,int *b,int *c,int *d){
  int ta[110],to[2010],co[2010],nt[2010],ca[2010],u[110];
  int i,j,k,min,mi,ans=0,id[100010],fe[110],fv[110];
  int h[110]={0};
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
	if(ca[i]&&  CO[to[i]]>CO[mi]+co[i]+h[mi]-h[to[i]]){
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
    ans+=h[t]*min;
    for(i=t;i-s;i=fv[i]){
      ca[fe[i]]-=min;
      ca[(fe[i]+e)%(2*e)]+=min;
    }
  }
  return ans;
}
int main(){
  int v,e,m,i,a[100010],b[100010],c[100010],d[100010];
  scanf("%d %d %d",&v,&e,&m);
  for(i=0;i<e;i++)scanf("%d %d %d %d",&a[i],&b[i],&c[i],&d[i]);
  printf("%d\n",mifl(v,e,m,0,v-1,a,b,c,d));
  return 0;
}