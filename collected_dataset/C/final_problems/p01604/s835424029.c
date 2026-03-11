#include<stdio.h>
#include<string.h>
int main(){
  int n,i,j,mi,d[110],c[110],f[110]={0};
  char s[110][110];
  scanf("%d%*c",&n);
  for(i=0;i<n;i++){
    fgets(s[i],100,stdin);
    s[i][strlen(s[i])-2]=0;
  }
  for(i=0;i<n;i++){
    d[i]=i+1;
    if(s[i][4]==' '){
      for(j=0;strcmp(s[i]+5,s[j]);j++);
      d[i]=j;
    }
  }
  //for(i=0;i<n;i++)printf("%s %d\n",s[i],d[i]);
  for(i=0;i<110;i++)c[i]=1e9;
  c[mi=0]=0;
  for(i=0;i<n;i++){//printf("%d %d\n",mi,c[mi]);
    f[mi]=1;
    if(c[d[mi]]>c[mi])c[d[mi]]=c[mi];
    if(c[mi+1]>c[mi])c[mi+1]=c[mi]+1;
    for(j=0;j<n;j++){
      if(f[j]==0&&(f[mi]||c[mi]>c[j]))mi=j;
    }
  }
  printf("%d\n",c[n]);
  return 0;
}