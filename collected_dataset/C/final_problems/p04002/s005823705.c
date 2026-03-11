#include<stdio.h>
long long R=1,C=1,H[2000010],N[2000010];
//評価関数（いまはMAX）
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
int lub(int l,int r,long long n,long long *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lub(l,m,n,d):lub(m+1,r,n,d);
}
int Y[]={1,1, 1,0,0, 0,-1,-1,-1};
int X[]={1,0,-1,1,0,-1, 1, 0,-1};
int main(){
  int h,w,n,i,j,ny,nx;
  long long a,b,c[10]={0};
  scanf("%d %d %d",&h,&w,&n);//printf("%d %d %d\n",h,w,n);
  for(i=0;i<n;i++){//printf("%d\n",i);
    scanf("%lld %lld",&a,&b);
    for(j=0;j<9;j++){
      ny=a+Y[j];
      nx=b+X[j];
      if(ny>=h||ny<2||nx>=w||nx<2)continue;//printf("%d %d\n",ny-1,nx-1);
      hin((ny-1)*w+nx-1);
    }//printf("\n");
  }//printf("a\n");
  if(n){
    b=N[hout()];
    for(j=1;C-1;){//printf("%lld %lld\n",b/w,b%w);
      a=N[hout()];
      if(b==a)j++;
      else{
	c[j]++;
	j=1;
      }
      b=a;
    }
    c[j]++;
  }
  c[0]=(h-2LL)*(w-2);
  for(i=1;i<10;i++)c[0]-=c[i];
  for(i=0;i<10;i++)printf("%lld\n",c[i]);
  return 0;
}
