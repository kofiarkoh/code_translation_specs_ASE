#include<stdio.h>
int gcd(int a,int b){return b?gcd(b,a%b):a;}
int lcm(int a,int b){return a*(b/gcd(a,b));}
int f(int *d,int r){
  int n=r,i;
  for(i=0;i<r;i++){
    if(d[i])break;
  }
  if(i==r)return 1;
  if(d[0]==0)return r;
  for(i=1;i<r;i++){
    if(d[i])n=gcd(n,i);
  }
  for(i=0;i<r;i++){
    if(i%n==0)d[i/n]=d[i];
  }
  return r/n;
}
int main(){
  int a,b,i,j,n,m,l,d[10][1500],r[10]={0};
  char s[2500],c[]={"0123456789ABCDEF"};
  scanf("%d",&m);
  while(m--){
    int ans[1500]={0};
    l=1;
    scanf("%d",&n);
    for(i=0;i<n;i++){
      scanf("%s",s);//printf("%d\n",s[2]);
      if(l>1024)continue;
      for(j=r[i]=0;s[j];j+=2){
	for(a=0;s[j  ]-c[a];a++);//printf("a%d\n",a);
	for(b=0;s[j+1]-c[b];b++);//printf("b%d\n",b);
	d[i][r[i]++]=a*16+b;//printf("%d\n",d[i][r[i]-1]);
      }//printf("\n");
      l=lcm(l,r[i]=f(d[i],r[i]));//printf("%d ",l);
    }//printf("\n");
    if(l>1024)printf("Too complex.\n");
    else{//printf("a\n");
      for(i=0;i<n;i++){
	for(j=0;j<r[i];j++)ans[l/r[i]*j]|=d[i][j];
      }//for(i=0;i<l;i++)printf("%x",ans[i]);printf("\n");
      l=f(ans,l);//printf("c\n");
      for(i=0;i<l;i++)printf("%02X",ans[i]);
      printf("\n");
    }
  }
  return 0;
}