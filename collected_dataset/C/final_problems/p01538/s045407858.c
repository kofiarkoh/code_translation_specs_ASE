#include<stdio.h>
int MAX(int a,int b){return a<b?b:a;}
void intstr(int n,char* s){
  int i,j=0;
  for(i=n?n:1;i  ;i/=10)s[++j]=0;
  for(i=n;j--;i/=10)s[j]=i%10+'0';
}
int strint(char* a){
  int n,i;
  for(i=n=0;a[i];i++)n=n*10+a[i]-'0';
  return n;
}
int main(){
  int i,n,t,max,m;
  char s[1010];
  scanf("%d",&m);
  while(m--){
    scanf("%s",s);//printf("%d\n",n);
    for(t=0;t<20;t++){
      if(s[1]==0)break;
      for(i=n=max=0;s[i+1];i++){
	n=n*10+s[i]-'0';//printf("%d %d:",n,strint(s+i+1));
	max=MAX(max,n*strint(s+i+1));
      }//printf(":%d  ",max);
      intstr(max,s);//printf("%s\n",s);
    }
    printf("%d\n",t-100000?t:-1);
  } 
  return 0;
}