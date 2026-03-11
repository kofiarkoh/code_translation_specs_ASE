#include<stdio.h>
#include<string.h>
int main(){
  int i,j,n,m,a,d[50],min;
  char s[5000],c[]={"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"};
  while(scanf("%d %s",&n,s),n){
    j=n;
    m=0;
    min=1000000000;
    for(i=2;i<=j;i++){
      while(j%i==0){
	j/=i;
	d[i]++;
      }
    }//printf("a\n");
    //for(i=0;i<n;i++)printf("%d %d\n",i,d[i]);
    for(i=0;s[i];i++){
      m*=n;
      for(j=0;c[j]-s[i];j++);
      m+=j;
    }//printf("%d\n",m);
    for(i=0;i<=n;i++){
      if(d[i]==0)continue;
      a=0;
      for(j=i;j<=m;j*=i)a+=m/j;
      a/=d[i];//printf("%d %d\n",i,a);
      if(min>a)min=a;
    }
  
  printf("%d\n",a);
  }
  return 0;
}
      