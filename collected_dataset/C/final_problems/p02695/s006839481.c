#include<stdio.h>
#include<string.h>
int nxpt(char *s){
  int i,l=strlen(s),max=0,f[260]={0};
  for(l--;l+1&&max<=s[l];l--)f[max=s[l]]++;
  if(l<0)return 0;
  f[s[l]]++;
  for(i=s[l]+1;f[i]==0;i++);
  f[s[l]=i]--;
  for(i=0;s[++l];f[s[l]=i]--){
    while(f[i]==0)i++;
  }
  return 1;
}
int main(){
  int n,m,q,a[100],b[100],c[100],d[100],e[100],i,j,ans=0,f,r;
  char s[100];
  scanf("%d %d %d",&n,&m,&q);
  for(i=0;i<q;i++)scanf("%d %d %d %d",&a[i],&b[i],&c[i],&d[i]);
  for(i=0;i<n;i++)s[i]='0';
  for(i=0;i<m;i++)s[i+n]='1';
  s[n+m]=0;
  do{
    for(i=j=r=0;s[i];i++){
      if(s[i]=='1')j++;
      else         e[r++]=j;
    }
    //for(i=0;i<r;i++)printf("%d ",e[i]);printf("\n");
    for(i=f=0;i<q;i++){
      if(e[b[i]-1]-e[a[i]-1]==c[i])f+=d[i];
    }
    if(ans<f)ans=f;
  }while(nxpt(s));
  printf("%d\n",ans);
  return 0;
}
