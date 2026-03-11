#include<stdio.h>
int R=1,C=1,H[2000010],N[2000010];
//ティツゥツ陛、ツセツ。テゥツ鳴「テヲツ閉ーテッツシツ暗」ツ??」ツ?セテ」ツ?ッMAXテッツシツ?
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
int lub(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<=d[m]?lub(l,m,n,d):lub(m+1,r,n,d);
}
int ub(int l,int r,int n,int *d){
  int m=(l+r)/2;
  if(l==r)return m;
  return n<d[m]?ub(l,m,n,d):ub(m+1,r,n,d);
}
int c[5010][5010];
int main(){
  int n,m,i,j,k,l,yr=0,xr=0,x[5010],y[5010],d[2][5010]={};
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++)scanf("%d %d",&x[i],&y[i]);
  for(i=0;i<n;i++)hin(x[i]);
  for(i=0;i<n;i++){
    d[0][xr++]=N[hout()];
    if(xr>1&&d[0][xr-1]==d[0][xr-2])xr--;
  }
  R=C=1;
  for(i=0;i<n;i++)hin(y[i]);
  for(i=0;i<n;i++){
    d[1][yr++]=N[hout()];
    if(yr>1&&d[1][yr-1]==d[1][yr-2])yr--;
  }
  //for(i=0;i<n;i++)printf("%d %d\n",d[0][i],d[1][i]);
  for(i=0;i<n;i++)c[lub(0,yr,y[i],d[1])+1][lub(0,xr,x[i],d[0])+1]++;
  for(i=1;i<=yr+1;i++){
    for(j=1;j<=xr+1;j++)c[i][j]+=c[i-1][j]+c[i][j-1]-c[i-1][j-1];
  }
  /*printf("        ");
  for(i=0;i<n+1;i++)printf("%3d ",d[0][i]);printf("\n");
  for(i=0;i<=n+1;i++){
    i?printf("%3d ",d[1][i-1]):printf("    ");
    for(j=0;j<=n+1;j++)printf("%3d ",c[i][j]);printf("\n");
    }//*/
  while(m--){
    scanf("%d %d %d %d",&i,&j,&k,&l);
    i=lub(0,xr,i,d[0]);
    j=lub(0,yr,j,d[1]);
    k=ub(0,xr,k,d[0]);
    l=ub(0,yr,l,d[1]);//printf("%d %d %d %d\n",i,j,k,l);
    printf("%d\n",c[j][i]-c[l][i]-c[j][k]+c[l][k]);
  }
  return 0;
}