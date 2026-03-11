#include<stdio.h>
int f(char *a,char *b){//is nihou
  int i,j;
  for(i=0;a[i];i++){
    for(j=0;b[j];j++){
      if(a[i+j]=='?'||b[j]=='?')continue;
      if(a[i+j]!=b[j])goto NEXT;
    }
    return 1;
  NEXT:;
  }
  return 0;
}
int g(char *a,char *b){
  int i,j,n,m,min;
  for(n=0;a[n];n++);
  for(m=0;b[m];m++);
  min=n+m;
  for(i=0;i<n;i++){
    for(j=0;j<m;j++){
      if(a[i+j]==0)break;
      if(a[i+j]=='?'||b[j]=='?')continue;
      if(a[i+j]!=b[j])goto NEXT;
    }
    if(min>n+i)min=n+i;
  NEXT:;
  }
  return min;
}
int MI(int a,int b){return  a<b?a:b;}
int MIN(int a,int b,int c){return MI(a,MI(b,c));}
int main(){
  char a[2010],b[2010],c[2010];
  int min=1e9,i,j,k;
  scanf("%s %s %s",a,b,c);
  for(i=0;a[i];i++);
  for(j=0;b[j];j++);
  for(k=0;c[k];k++);
  if(f(a,b))min=MIN(min,g(a,c),g(c,a));//printf("%d\n",min);
  if(f(b,a))min=MIN(min,g(b,c),g(c,b));//printf("%d\n",min);

  if(f(b,c))min=MIN(min,g(b,a),g(a,b));//printf("%d\n",min);
  if(f(c,b))min=MIN(min,g(c,a),g(a,c));//printf("%d\n",min);
    
  if(f(a,c))min=MIN(min,g(a,b),g(b,a));//printf("%d\n",min);
  if(f(c,a))min=MIN(min,g(c,b),g(b,c));//printf("%d\n",min);

  min=MIN(min,g(a,b)+g(b,c)-j,g(a,c)+g(c,b)-k);//printf("%d\n",min);
  min=MIN(min,g(b,c)+g(c,a)-k,g(b,a)+g(a,c)-i);//printf("%d\n",min);
  min=MIN(min,g(c,a)+g(a,b)-i,g(c,b)+g(b,a)-j);//printf("%d\n",min);
  printf("%d\n",min);
  return 0;
}
