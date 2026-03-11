#include<stdio.h>
#include<string.h>
int main(){
  int n,i,j,a[10010],b,c,y,m,d;
  char s[4][110],t[110][110],o[110][110];
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%s %s %d/%d/%d",t[i],o[i],&y,&m,&d);
    a[i]=y*10000+m*100+d;
  }
  scanf("%d",&m);
  while(m--){
    for(i=0;i<4;i++)scanf("%s",s[i]);//printf("%s %s %S %s\n");
    for(i=0;i<n;i++){
      if(s[0][0]-'*'&&strstr(t[i],s[0])==NULL)continue;
      if(s[1][0]-'*'&&strstr(o[i],s[1])==NULL)continue;
      for(j=b=0;s[2][j];j++)b=s[2][j]-'/'?b*10+s[2][j]-'0':b;
      if(s[2][0]=='*')b=0;
      for(j=c=0;s[3][j];j++)c=s[3][j]-'/'?c*10+s[3][j]-'0':c;
      if(s[3][0]=='*')c=1e9;
      if(b<=a[i]&&a[i]<=c)printf("%s\n",t[i]);
    }
    if(m)printf("\n");
  }
  return 0;
}