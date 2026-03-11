#include<stdio.h>
int l;
void f(int n,int a[20]){//kakunou
  int i;
  for(i=0;i<l;n/=3)a[i++]=n%3;
}
int g(int a[20]){
  int n=0,i;
  for(i=l;i;i--)n=n*3+a[i-1];
return n;
}
int zMIN(int a,int b){
  if(b==0)return a;
  if(a==0||a>b)return b;
  return a;
}
int main(){
  int i,j,k,n,a[20];
  char s[20];
  while(scanf("%s",s),s[1]){
    int d[60000]={0};
    for(l=n=0;s[l];l++){
      if(s[l]=='r')n=n*3+0;
      if(s[l]=='g')n=n*3+1;
      if(s[l]=='b')n=n*3+2;
    }
    d[n]=1;
    for(i=n=0;i<l;i++)n=n*3+2;//printf("%d\n",n);
    for(i=0;i<25;i++){
      for(j=0;j<=n;j++){
    if(d[j]==0)continue;
    for(k=1;k<l;k++){
      f(j,a);
      a[k]=a[k-1]=(6-a[k]-a[k-1])%3;
      d[g(a)]=zMIN(d[g(a)],d[j]+1);
    }
      }
    }
    d[0]=zMIN(d[0],zMIN(d[n/2],d[n]));
    if(d[0])printf("%d\n",d[0]-1);
    else    printf("NA\n");
  }
  return 0;
}